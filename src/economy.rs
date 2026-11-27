//! P2-X1: integer market rules, independent of rendering and navigation.

pub const GOODS: [&str; 3] = ["SUCRE", "BOIS", "OUTILS"];
pub const CAPACITY: u8 = 30;
pub const STOCK_LIMIT: u8 = 40;
pub const START_CASH: u64 = 300;
pub const TICKS_PER_DAY: u16 = 200;
const INITIAL: [[u8; 3]; 3] = [[32, 8, 12], [12, 32, 10], [0, 12, 32]];
const EQUILIBRIUM: [[u8; 3]; 3] = [[32, 8, 12], [8, 32, 10], [4, 12, 32]];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeError {
    Invalid,
    Stock,
    Cargo,
    Capacity,
    Cash,
    Stale,
    Overflow,
}

impl TradeError {
    pub fn message(self) -> &'static str {
        match self {
            Self::Invalid => "QUANTITE OU MARCHE INVALIDE",
            Self::Stock => "STOCK INSUFFISANT / PORT PLEIN",
            Self::Cargo => "MARCHANDISE ABSENTE DE LA CALE",
            Self::Capacity => "CALE INSUFFISANTE",
            Self::Cash => "OR INSUFFISANT",
            Self::Stale => "DEVIS EXPIRE",
            Self::Overflow => "LIMITE COMPTABLE",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quote {
    port: usize,
    good: usize,
    side: Side,
    quantity: u8,
    total: u64,
    day: u64,
    revision: u64,
}

impl Quote {
    pub fn total(self) -> u64 {
        self.total
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Observation {
    pub day: u64,
    pub buy: [Option<u64>; 3],
    pub sell: [Option<u64>; 3],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Economy {
    cash: u64,
    cargo: [u8; 3],
    stocks: [[u8; 3]; 3],
    day: u64,
    sea_ticks: u16,
    revision: u64,
    observations: [Option<Observation>; 3],
}

impl Default for Economy {
    fn default() -> Self {
        let mut economy = Self {
            cash: START_CASH,
            cargo: [0; 3],
            stocks: INITIAL,
            day: 0,
            sea_ticks: 0,
            revision: 0,
            observations: [None; 3],
        };
        economy.observe(0);
        economy
    }
}

// k denotes the LOWER stock of the unit interval, in both directions.
fn unit(k: u8, side: Side) -> u64 {
    let n = 10 * (80 - 2 * i32::from(k)).clamp(20, 80) as u64;
    match side {
        Side::Buy => (11 * n).div_ceil(400),
        Side::Sell => 9 * n / 400,
    }
}

impl Economy {
    pub fn cash(&self) -> u64 {
        self.cash
    }

    pub fn cargo(&self) -> [u8; 3] {
        self.cargo
    }

    pub fn used(&self) -> u8 {
        self.cargo.iter().sum()
    }

    pub fn day(&self) -> u64 {
        self.day
    }

    pub fn sea_ticks(&self) -> u16 {
        self.sea_ticks
    }

    pub fn stock(&self, port: usize, good: usize) -> Option<u8> {
        self.stocks.get(port)?.get(good).copied()
    }

    pub fn marginal(&self, port: usize, good: usize, side: Side) -> Option<u64> {
        let s = self.stock(port, good)?;
        match side {
            Side::Buy if s > 0 => Some(unit(s - 1, side)),
            Side::Sell if s < STOCK_LIMIT => Some(unit(s, side)),
            _ => None,
        }
    }

    pub fn observation(&self, port: usize) -> Option<Observation> {
        self.observations.get(port).copied().flatten()
    }

    pub fn observe(&mut self, port: usize) {
        if port >= self.stocks.len() {
            return;
        }
        let observation = Observation {
            day: self.day,
            buy: std::array::from_fn(|good| self.marginal(port, good, Side::Buy)),
            sell: std::array::from_fn(|good| self.marginal(port, good, Side::Sell)),
        };
        self.observations[port] = Some(observation);
    }

    pub fn sea_tick(&mut self) {
        self.sea_ticks += 1;
        if self.sea_ticks < TICKS_PER_DAY {
            return;
        }
        self.sea_ticks = 0;
        self.day = self.day.saturating_add(1);
        for (row, target) in self.stocks.iter_mut().zip(EQUILIBRIUM) {
            for (s, e) in row.iter_mut().zip(target) {
                if *s < e {
                    *s += 1;
                } else if *s > e {
                    *s -= 1;
                }
            }
        }
    }

    pub fn quote(
        &self,
        port: usize,
        good: usize,
        side: Side,
        quantity: u8,
    ) -> Result<Quote, TradeError> {
        let s = self.stock(port, good).ok_or(TradeError::Invalid)?;
        if quantity == 0 || quantity > CAPACITY {
            return Err(TradeError::Invalid);
        }
        let range = match side {
            Side::Buy => {
                if quantity > s {
                    return Err(TradeError::Stock);
                }
                if quantity > CAPACITY - self.used() {
                    return Err(TradeError::Capacity);
                }
                s - quantity..s
            }
            Side::Sell => {
                if quantity > self.cargo[good] {
                    return Err(TradeError::Cargo);
                }
                if quantity > STOCK_LIMIT - s {
                    return Err(TradeError::Stock);
                }
                s..s + quantity
            }
        };
        let total = range.map(|k| unit(k, side)).sum();
        match side {
            Side::Buy if total > self.cash => return Err(TradeError::Cash),
            Side::Sell if self.cash.checked_add(total).is_none() => {
                return Err(TradeError::Overflow);
            }
            _ => {}
        }
        Ok(Quote {
            port,
            good,
            side,
            quantity,
            total,
            day: self.day,
            revision: self.revision,
        })
    }

    /// Quotes are single-use and bound to the economic state. The adapter
    /// additionally checks actual docking before calling this method.
    pub fn execute(&mut self, quote: Quote) -> Result<(), TradeError> {
        if quote.day != self.day || quote.revision != self.revision {
            return Err(TradeError::Stale);
        }
        let current = self.quote(quote.port, quote.good, quote.side, quote.quantity)?;
        if current != quote {
            return Err(TradeError::Stale);
        }
        let revision = self.revision.checked_add(1).ok_or(TradeError::Overflow)?;
        let cash = match quote.side {
            Side::Buy => self.cash.checked_sub(quote.total),
            Side::Sell => self.cash.checked_add(quote.total),
        }
        .ok_or(TradeError::Overflow)?;
        // All potentially failing operations precede mutation.
        let stock = &mut self.stocks[quote.port][quote.good];
        let cargo = &mut self.cargo[quote.good];
        match quote.side {
            Side::Buy => {
                *stock -= quote.quantity;
                *cargo += quote.quantity;
            }
            Side::Sell => {
                *stock += quote.quantity;
                *cargo -= quote.quantity;
            }
        }
        self.cash = cash;
        self.revision = revision;
        self.observe(quote.port);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_quotes_and_round_trip_are_exact() {
        let mut e = Economy::default();
        let buy = e.quote(0, 0, Side::Buy, 3).unwrap();
        assert_eq!(buy.total(), 19);
        e.execute(buy).unwrap();
        let sell = e.quote(0, 0, Side::Sell, 3).unwrap();
        assert_eq!(sell.total(), 12);
        e.execute(sell).unwrap();
        assert_eq!(e.cash(), 293);
        assert_eq!(e.stock(0, 0), Some(32));
        assert_eq!(e.used(), 0);
        for s in 1..=STOCK_LIMIT {
            for q in 1..=s.min(CAPACITY) {
                let buy: u64 = (s - q..s).map(|k| unit(k, Side::Buy)).sum();
                let sell: u64 = (s - q..s).map(|k| unit(k, Side::Sell)).sum();
                assert!(buy > sell);
            }
        }
    }

    #[test]
    fn invalid_and_stale_quotes_are_atomic() {
        let mut e = Economy::default();
        assert_eq!(e.quote(2, 0, Side::Buy, 1), Err(TradeError::Stock));
        assert_eq!(e.quote(0, 0, Side::Sell, 1), Err(TradeError::Cargo));
        assert_eq!(e.quote(0, 0, Side::Buy, 0), Err(TradeError::Invalid));
        let q = e.quote(0, 0, Side::Buy, 8).unwrap();
        e.execute(q).unwrap();
        let snapshot = e.clone();
        assert_eq!(e.execute(q), Err(TradeError::Stale));
        assert_eq!(e, snapshot);
        assert_eq!(e.quote(0, 0, Side::Buy, 23), Err(TradeError::Capacity));
        let q = e.quote(0, 0, Side::Buy, 1).unwrap();
        for _ in 0..TICKS_PER_DAY {
            e.sea_tick();
        }
        let snapshot = e.clone();
        assert_eq!(e.execute(q), Err(TradeError::Stale));
        assert_eq!(e, snapshot);
    }

    #[test]
    fn cash_capacity_and_port_limit_are_enforced() {
        let mut e = Economy {
            cash: 0,
            ..Economy::default()
        };
        assert_eq!(e.quote(0, 0, Side::Buy, 1), Err(TradeError::Cash));
        e.cash = 300;
        e.cargo = [30, 0, 0];
        assert_eq!(e.quote(0, 1, Side::Buy, 1), Err(TradeError::Capacity));
        assert_eq!(e.quote(0, 0, Side::Sell, 9), Err(TradeError::Stock));
        let quote = e.quote(0, 0, Side::Sell, 8).unwrap();
        e.execute(quote).unwrap();
        assert_eq!(e.stock(0, 0), Some(40));
        assert_eq!(e.used(), 22);
        assert_eq!(e.quote(0, 0, Side::Sell, 1), Err(TradeError::Stock));
    }

    #[test]
    fn restoration_preserves_observed_prices_until_visit() {
        let mut e = Economy::default();
        let observation = e.observation(0);
        assert_eq!(e.observation(1), None);
        for _ in 0..199 {
            e.sea_tick();
        }
        assert_eq!(e.day(), 0);
        e.sea_tick();
        assert_eq!(e.day(), 1);
        assert_eq!(e.stock(1, 0), Some(11));
        assert_eq!(e.stock(2, 0), Some(1));
        assert_eq!(e.observation(0), observation);
        assert_eq!(e.observation(1), None);
        e.observe(1);
        assert_eq!(e.observation(1).unwrap().day, 1);
    }
}
