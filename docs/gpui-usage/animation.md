# Animation

**Components:** [`Animation`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/animation.rs), [`AnimationElement`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/animation.rs), [`AnimationExt`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/animation.rs)

## What is the component and what it does

GPUI's animation system wraps any element with time-based property interpolation. You provide an `Animation` (duration, easing, loop mode) and an `animator` closure that receives the element and a `delta` value (0.0–1.0) representing progress. The animator returns the element with modified properties.

Animations are driven by `window.request_animation_frame()` — each frame, the delta is recalculated and the animator closure is called with the updated value.

## Preconditions for use

```rust
use gpui::{Animation, AnimationExt};
use gpui::{ease_in_out, linear, quadratic, bounce}; // Easing functions
use std::time::Duration;
```

- `AnimationExt` trait must be in scope — it's implemented for all `IntoElement` types
- The animation needs a unique `ElementId` for state tracking across frames
- Easing functions are re-exported at the `gpui` crate root (e.g., `gpui::ease_in_out`)

## Signature for usage

### Animation builder

```rust
Animation::new(Duration::from_secs(1))          // One-shot, linear, 1 second
    .repeat()                                     // Loop indefinitely
    .with_easing(ease_in_out)                     // Custom easing curve
```

### Fields

```rust
pub struct Animation {
    pub duration: Duration,
    pub oneshot: bool,           // true = play once, false = loop
    pub easing: Rc<dyn Fn(f32) -> f32>,
}
```

### Applying animation to an element

```rust
// Single animation
element
    .with_animation(
        "unique-id",                              // ElementId for state tracking
        Animation::new(Duration::from_millis(300)),
        |element, delta| {                        // delta: 0.0 → 1.0
            element.left(px(delta * 100.0))       // Animate position
        },
    )

// Multiple sequential animations
element
    .with_animations(
        "multi-anim",
        vec![anim1, anim2, anim3],
        |element, anim_index, delta| {            // Which animation + progress
            match anim_index {
                0 => element.left(px(delta * 100.0)),
                1 => element.top(px(delta * 50.0)),
                _ => element,
            }
        },
    )
```

### Available easing functions

```rust
linear(delta: f32) -> f32        // Constant speed
quadratic(delta: f32) -> f32     // Accelerating (slow start)
ease_in_out(delta: f32) -> f32   // Slow at start and end, fast in middle
ease_out_quint() -> impl Fn(f32) -> f32   // Fast start, slow end (returns closure)
bounce(easing) -> impl Fn(f32) -> f32     // Forward then reverse (wraps another easing)
pulsating_between(min, max) -> impl Fn(f32) -> f32  // Breathing/pulse effect
```

All easing functions map `[0.0, 1.0] → [0.0, 1.0]` with `f(0) ≈ 0` and `f(1) ≈ 1` (except `pulsating_between` which oscillates).

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `AnimationExt` | Provides `.with_animation()` and `.with_animations()` on all elements |
| `IntoElement` | Elements must implement this to be animated |

## Usage and examples

### Fade-in position animation

```rust
impl Render for AnimatedView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            div()
                .size_8()
                .bg(gpui::red())
                .with_animation(
                    "slide-in",
                    Animation::new(Duration::from_millis(300))
                        .with_easing(ease_in_out),
                    |el, delta| el.left(px(delta * 200.0)),
                ),
        )
    }
}
```

### Looping animation

```rust
div()
    .size_8()
    .bg(gpui::blue())
    .with_animation(
        "pulse",
        Animation::new(Duration::from_secs(2)).repeat(),
        |el, delta| el.left(px(delta * 100.0)),
    )
```

### SVG rotation (from Zed's animation example)

```rust
use gpui::{svg, Transformation, percentage, bounce, ease_in_out};

svg()
    .size_20()
    .with_animation(
        "rotate",
        Animation::new(Duration::from_secs(2))
            .repeat()
            .with_easing(bounce(ease_in_out)),
        |svg, delta| {
            svg.with_transformation(Transformation::rotate(percentage(delta)))
        },
    )
```

### Testing easing functions (pure math)

```rust
#[test]
fn test_easing_linear() {
    assert_eq!(linear(0.0), 0.0);
    assert_eq!(linear(0.5), 0.5);
    assert_eq!(linear(1.0), 1.0);
}

#[test]
fn test_easing_ease_in_out() {
    assert!((ease_in_out(0.0) - 0.0).abs() < 0.01);
    assert!((ease_in_out(1.0) - 1.0).abs() < 0.01);
    // Midpoint should be approximately 0.5
    assert!((ease_in_out(0.5) - 0.5).abs() < 0.1);
}

#[test]
fn test_quadratic_slower_at_start() {
    assert!(quadratic(0.25) < 0.25);  // Below linear at start
}
```

## Post-conditions / destruction requirements

- Animations are cleaned up when the element is removed from the tree
- Looping animations (`repeat()`) run until the element is dropped
- One-shot animations stop at `delta = 1.0` and hold the final state
- Animation state is keyed by `ElementId` — reusing an ID resumes the animation
- No explicit stop/cancel API — remove the element to stop the animation

## Testing

Easing functions are pure `f32 → f32` and fully testable. Animation elements can be created in test windows:

```rust
#[gpui::test]
fn test_animation_in_window(cx: &mut TestAppContext) {
    struct AnimView;
    impl Render for AnimView {
        fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                div().size_8().bg(gpui::red())
                    .with_animation("test", Animation::new(Duration::from_secs(1)),
                        |el, delta| el.left(px(delta * 100.0)))
            )
        }
    }
    let _window = cx.add_window(|_w, _cx| AnimView);
}
```

Run tests: `cargo test --test animation_test`

## Surprises, Anti-patterns, and Bugs

### Animations re-render every frame

An animated element calls `request_animation_frame()` each frame, causing continuous re-rendering of the entire view. This has a performance cost — avoid animating many elements simultaneously.

### No keyframe or sequence support built-in

GPUI's animation is interpolation-only (start → end). For multi-step sequences, use `.with_animations()` with multiple `Animation` objects. There's no CSS-like `@keyframes` equivalent.

### Animation state resets if ElementId changes

The animation progress is tracked by `ElementId`. If you change the ID (e.g., generate a new one each render), the animation restarts from the beginning.

### `easing` functions are at crate root, not a submodule

Import as `gpui::ease_in_out`, not `gpui::easing::ease_in_out`. The easing module is re-exported with `pub use easing::*` at the elements level, which propagates to the crate root.

### `ease_out_quint()` returns a closure, not a value

Unlike `linear` and `ease_in_out` which are functions, `ease_out_quint()` and `bounce()` return closures. Use them like: `.with_easing(ease_out_quint())` or `.with_easing(bounce(ease_in_out))`.

### Oneshot animations hold final state

A one-shot animation (default) reaches `delta = 1.0` and stays there. The animator closure is called one final time with `delta = 1.0`, and the element holds that state permanently.

### `fade_out` on Hsla is `&mut self`, not a builder

`Hsla::fade_out(&mut self, factor)` mutates in place and returns `()`. You cannot use it in a builder chain. Instead, create a color and modify it:

```rust
let mut color = gpui::red();
color.fade_out(delta);
el.bg(color)
```
