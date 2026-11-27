// Optional real-browser acceptance aid. Requires an existing Playwright install.
// First package_web.py, then serve dist on localhost:8765 in another terminal.
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');
const root = path.resolve(__dirname, '..');
const output = path.join(root, 'target/browser-check');
fs.mkdirSync(output, { recursive: true });
function command(name, args) {
  return execFileSync(name, args, { cwd: root, encoding: 'utf8' });
}

(async () => {
  const native = command('cargo', ['test', '--locked', '--example', 'navigation_probe', '--', '--nocapture']);
  const expected = native.match(/REFERENCE_STATE=(.*)/)[1];
  command('cargo', ['build', '--locked', '--release', '--target', 'wasm32-unknown-unknown', '--example', 'navigation_probe']);
  const target = process.env.CARGO_TARGET_DIR || path.join(root, 'target');
  command('wasm-bindgen', ['--target', 'web', '--out-dir', 'dist/probe', '--out-name', 'probe', path.join(target, 'wasm32-unknown-unknown/release/examples/navigation_probe.wasm')]);
  const browser = await chromium.launch({
    executablePath: process.env.CHROMIUM_EXECUTABLE || undefined,
    headless: process.env.HEADED !== '1',
    args: process.env.SOFTWARE_WEBGPU === '1' ? ['--enable-unsafe-webgpu', '--use-angle=swiftshader', '--use-vulkan=swiftshader', '--enable-features=Vulkan'] : [],
  });
  try {
    const page = await browser.newPage({ viewport: { width: 1500, height: 950 } });
    const errors = [];
    page.on('pageerror', error => errors.push(String(error)));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await page.addInitScript(() => {
      globalThis.__frameTimes = [];
      const submit = GPUQueue.prototype.submit;
      GPUQueue.prototype.submit = function (...args) {
        if (globalThis.__frameTimes.length < 2000) globalThis.__frameTimes.push(performance.now());
        return submit.apply(this, args);
      };
    });
    await page.goto(process.env.GAME_URL || 'http://127.0.0.1:8765');
    await page.waitForFunction(() => globalThis.__frameTimes.length > 30);
    const actual = await page.evaluate(async () => {
      const probe = await import('/probe/probe.js'); await probe.default(); return probe.reference_state();
    });
    assert.equal(actual, expected, 'native and WASM full states differ');
    const actualTrade = await page.evaluate(async () => (await import('/probe/probe.js')).trade_state());
    assert.equal(actualTrade, expectedTrade, 'native and WASM economy states differ');
    const canvas = page.locator('canvas');
    await canvas.click();
    const capture = name => canvas.screenshot({ path: path.join(output, name + '.png') });
    const press = async key => {
      await page.keyboard.down(key); await page.waitForTimeout(60);
      await page.keyboard.up(key); await page.waitForTimeout(60);
    };
    const turn = async (key, count) => { await press('ArrowDown'); for (let i = 0; i < count; i++) await press(key); };
    const initial = await capture('web-initial');
    await press('Enter');
    const market = await capture('web-market');
    assert.notDeepEqual(market, initial, 'market did not open');
    await press('Enter');
    const purchased = await capture('web-market-buy');
    assert.notDeepEqual(purchased, market, 'buy did not change market');
    await page.waitForTimeout(200);
    assert.deepEqual(await canvas.screenshot(), purchased, 'market did not freeze');
    await press('Tab');
    const sellQuote = await capture('web-market-sell-quote');
    assert.notDeepEqual(sellQuote, purchased, 'sale selection did not change');
    await press('Enter');
    assert.notDeepEqual(await capture('web-market-sell'), sellQuote, 'sell did not change market');
    await press('Escape');
    await press('r');
    // Full chart freezes rendering and rejects a held rudder on return.
    await press('ArrowUp'); await page.waitForTimeout(300);
    await press('m');
    const chart = await capture('web-chart'); await page.waitForTimeout(200);
    assert.deepEqual(await canvas.screenshot(), chart, 'chart did not pause');
    await page.keyboard.down('ArrowRight'); await press('m');
    const latched = await capture('web-return-held'); await page.waitForTimeout(250);
    assert.notDeepEqual(await canvas.screenshot(), latched, 'held rudder caused a hidden pause');
    await page.keyboard.up('ArrowRight'); await page.waitForTimeout(250);
    assert.notDeepEqual(await canvas.screenshot(), latched, 'neutral input did not resume');
    await press('r');

    await press('ArrowUp'); await page.waitForTimeout(500); await press('Escape');
    const paused = await capture('web-pause'); await page.waitForTimeout(200);
    await press('m'); await capture('web-chart-from-pause'); await press('m');
    assert.deepEqual(await canvas.screenshot(), paused, 'chart discarded prior pause');
    assert.deepEqual(await canvas.screenshot(), paused, 'paused ship changed');
    await press('Escape'); await page.waitForTimeout(200);
    assert.notDeepEqual(await canvas.screenshot(), paused, 'resume did not move the ship');
    await press('r');
    await turn('ArrowRight', 2); await press('ArrowUp'); await page.waitForTimeout(5420); await press('ArrowDown');
    await capture('route-south');
    await turn('ArrowLeft', 2); await press('ArrowUp'); await page.waitForTimeout(7830); await press('ArrowDown');
    await capture('route-east');
    await turn('ArrowLeft', 4); await press('ArrowUp'); await page.waitForTimeout(4800);
    await capture('web-arrival');
    // Inspect this capture for ARRIVEE PORT-PHARE. Human play feel is separate.
    await press('r');
    await page.setViewportSize({ width: 1000, height: 640 });
    await canvas.evaluate(element => { element.style.width = '960px'; element.style.height = '540px'; });
    await page.waitForTimeout(500); await capture('web-1x');
    await page.setViewportSize({ width: 1980, height: 1200 });
    await canvas.evaluate(element => { element.style.width = '1920px'; element.style.height = '1080px'; });
    await page.waitForTimeout(500); await capture('web-2x');
    await page.setViewportSize({ width: 1030, height: 720 });
    await canvas.evaluate(element => { element.style.width = '970px'; element.style.height = '580px'; });
    await page.waitForTimeout(500); await capture('web-resize');
    const gpu = await page.evaluate(async () => {
      const adapter = await navigator.gpu.requestAdapter();
      return { vendor: adapter.info.vendor, device: adapter.info.device, architecture: adapter.info.architecture, description: adapter.info.description };
    });
    const times = await page.evaluate(() => globalThis.__frameTimes);
    const frameMs = (times.at(-1) - times[0]) / (times.length - 1);
    assert.deepEqual(errors, []);
    console.log(JSON.stringify({ browser: browser.version(), gpu, nativeWasmState: actual, nativeWasmTradeState: actualTrade, frames: times.length, meanFrameMs: frameMs, screenshots: output, errors }, null, 2));
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
