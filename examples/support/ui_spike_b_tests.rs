use super::*;
use gotoo_pixel_engine::{Touch, TouchPhase};

fn nav(nav: UiNavInput) -> UiInput<'static> {
    UiInput {
        nav,
        ..Default::default()
    }
}
fn pointer(position: (i32, i32), pressed: bool, released: bool) -> UiInput<'static> {
    UiInput {
        pointer: UiPointerInput {
            position: Some(position),
            pressed,
            released,
        },
        ..Default::default()
    }
}
fn tick(spike: &mut Spike, input: UiInput<'_>) -> ResolvedOutput {
    spike.interact(input, Duration::from_millis(16))
}

#[test]
fn scene_resources_and_all_labels_fit_visible_disjoint_geometry() {
    let mut s = Spike::new();
    assert!(s.font.supports_text("Échange — quantité, coût, cargaison"));
    assert!(s.image.width() > 0);
    for (i, t) in s.targets.iter().enumerate() {
        assert!(t.rect.x >= 0 && t.rect.y >= 0);
        assert!(t.rect.x as u32 + t.rect.width <= 960);
        assert!(t.rect.y as u32 + t.rect.height <= 540);
        assert!(
            s.targets[..i]
                .iter()
                .all(|other| !other.rect.intersects(t.rect))
        );
        let bounds = Rect {
            x: 0,
            y: 0,
            width: t.rect.width - 16,
            height: t.rect.height - 16,
        };
        let label = if t.enabled { LABELS[i] } else { "Indisponible" };
        let measured = s.font.measure(label, 22.0, bounds);
        assert!(measured.width <= bounds.width && measured.height <= bounds.height);
    }
}

#[test]
fn four_cardinal_round_trips_never_dispatch() {
    for (outward, back, index) in [
        (
            UiNavInput {
                up: true,
                ..Default::default()
            },
            UiNavInput {
                down: true,
                ..Default::default()
            },
            1,
        ),
        (
            UiNavInput {
                right: true,
                ..Default::default()
            },
            UiNavInput {
                left: true,
                ..Default::default()
            },
            2,
        ),
        (
            UiNavInput {
                down: true,
                ..Default::default()
            },
            UiNavInput {
                up: true,
                ..Default::default()
            },
            3,
        ),
        (
            UiNavInput {
                left: true,
                ..Default::default()
            },
            UiNavInput {
                right: true,
                ..Default::default()
            },
            4,
        ),
    ] {
        let mut s = Spike::new();
        s.targets[3].enabled = true;
        let center = s.targets[0].id;
        assert_eq!(tick(&mut s, UiInput::default()).focused, Some(center));
        let destination = s.targets[index].id;
        assert_eq!(tick(&mut s, nav(outward)).focused, Some(destination));
        assert_eq!(tick(&mut s, nav(back)).focused, Some(center));
        assert_eq!(s.count, 0);
    }
}

#[test]
fn exact_painted_edges_capture_and_dispatch_once() {
    let mut s = Spike::new();
    let center = s.targets[0].id;
    assert!(tick(&mut s, pointer((330, 220), true, false)).active(center));
    // Idle while held cannot dispatch another action.
    assert!(tick(&mut s, UiInput::default()).active(center));
    tick(&mut s, pointer((529, 289), false, true));
    assert_eq!(s.count, 1);
    tick(&mut s, pointer((529, 289), false, true));
    assert_eq!(s.count, 1);
    tick(&mut s, pointer((530, 289), true, false));
    tick(&mut s, pointer((530, 289), false, true));
    assert_eq!(s.count, 1);
    tick(&mut s, pointer((340, 230), true, false));
    tick(&mut s, pointer((600, 330), false, true));
    assert_eq!(s.count, 1);
}

#[test]
fn disabled_target_and_navigation_confirm_conflict_do_not_activate() {
    let mut s = Spike::new();
    tick(&mut s, UiInput::default());
    tick(&mut s, pointer((310, 380), true, false));
    tick(&mut s, pointer((310, 380), false, true));
    assert_eq!(s.count, 0);
    tick(
        &mut s,
        nav(UiNavInput {
            right: true,
            confirm: true,
            ..Default::default()
        }),
    );
    assert_eq!(s.count, 0);
    tick(
        &mut s,
        nav(UiNavInput {
            confirm: true,
            ..Default::default()
        }),
    );
    assert_eq!(s.count, 1);
    tick(&mut s, UiInput::default());
    assert_eq!(s.count, 1);
}

#[test]
fn cancel_and_suspend_drop_capture_until_neutral_then_fresh_press() {
    for cancel in [true, false] {
        let mut s = Spike::new();
        tick(&mut s, pointer((340, 230), true, false));
        if cancel {
            tick(
                &mut s,
                nav(UiNavInput {
                    cancel: true,
                    confirm: true,
                    ..Default::default()
                }),
            );
        } else {
            s.state.suspend();
        }
        let output = tick(&mut s, pointer((340, 230), false, true));
        assert!(!output.active(s.targets[0].id));
        assert_eq!(s.count, 0);
        tick(
            &mut s,
            nav(UiNavInput {
                confirm: true,
                ..Default::default()
            }),
        );
        assert_eq!(s.count, 0);
        tick(&mut s, UiInput::default());
        tick(&mut s, pointer((340, 230), true, false));
        tick(&mut s, pointer((340, 230), false, true));
        assert_eq!(s.count, 1);
    }
}

#[test]
fn touch_cancel_and_ambiguous_ends_dispatch_nothing() {
    let mut s = Spike::new();
    let touch = |id, phase, position| Touch {
        id,
        phase,
        position: Some(position),
    };
    for phase in [TouchPhase::Cancelled, TouchPhase::Moved] {
        tick(
            &mut s,
            UiInput {
                touches: &[touch(1, TouchPhase::Started, (340, 230))],
                ..Default::default()
            },
        );
        tick(
            &mut s,
            UiInput {
                touches: &[touch(1, phase, (600, 330))],
                ..Default::default()
            },
        );
        tick(
            &mut s,
            UiInput {
                touches: &[touch(1, TouchPhase::Ended, (340, 230))],
                ..Default::default()
            },
        );
    }
    assert_eq!(s.count, 0);
    tick(
        &mut s,
        UiInput {
            touches: &[
                touch(1, TouchPhase::Started, (340, 230)),
                touch(2, TouchPhase::Started, (670, 220)),
            ],
            ..Default::default()
        },
    );
    tick(
        &mut s,
        UiInput {
            touches: &[
                touch(1, TouchPhase::Ended, (340, 230)),
                touch(2, TouchPhase::Ended, (670, 220)),
            ],
            ..Default::default()
        },
    );
    assert_eq!(s.count, 0);
}

#[test]
fn real_input_focus_reset_even_between_frames_clears_capture_and_flash() {
    for coalesced in [false, true] {
        let mut s = Spike::new();
        let mut fb = Framebuffer::new(960, 540);
        let mut input = Input::default();
        tick(
            &mut s,
            nav(UiNavInput {
                confirm: true,
                ..Default::default()
            }),
        );
        assert!(s.flash.is_some());
        tick(&mut s, pointer((340, 230), true, false));
        input.set_window_focused(false);
        if coalesced {
            input.set_window_focused(true);
        }
        s.render(&mut fb, &input, Duration::ZERO);
        assert!(s.flash.is_none());
        input.set_window_focused(true);
        s.render(&mut fb, &input, Duration::ZERO);
        let output = tick(&mut s, pointer((340, 230), false, true));
        assert!(!output.active(s.targets[0].id));
        assert_eq!(s.count, 1);
        tick(&mut s, UiInput::default());
        tick(&mut s, pointer((340, 230), true, false));
        tick(&mut s, pointer((340, 230), false, true));
        assert_eq!(s.count, 2);
    }
}

#[test]
fn painter_distinguishes_focus_hover_capture_unavailable_and_confirmation() {
    let mut s = Spike::new();
    let mut fb = Framebuffer::new(960, 540);
    let pixel = |fb: &Framebuffer, x: usize, y: usize| {
        let offset = (y * 960 + x) * 4;
        fb.as_rgba8()[offset..offset + 3].to_vec()
    };
    let output = tick(&mut s, UiInput::default());
    s.paint(&mut fb, &output);
    assert_eq!(pixel(&fb, 330, 220), [120, 235, 255]);
    assert_eq!(pixel(&fb, 520, 280), [30, 43, 60]);
    assert_eq!(pixel(&fb, 550, 430), [58, 58, 58]);
    let output = tick(&mut s, pointer((340, 230), false, false));
    s.paint(&mut fb, &output);
    assert_eq!(pixel(&fb, 520, 280), [45, 75, 110]);
    let output = tick(&mut s, pointer((340, 230), true, false));
    s.paint(&mut fb, &output);
    assert_eq!(pixel(&fb, 520, 280), [50, 145, 120]);
    let output = tick(&mut s, pointer((340, 230), false, true));
    s.paint(&mut fb, &output);
    assert_eq!(pixel(&fb, 330, 220), [255, 180, 70]);
    assert_eq!(s.count, 1);
}
