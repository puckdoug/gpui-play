# Animation

## Goal

Document `AnimationElement` and easing functions (linear, ease_in, ease_out, ease_in_out, bounce, etc.).

## Design

A showcase view with multiple animated elements, each using a different easing curve. A "Play" button triggers all animations simultaneously so curves can be compared visually. A second section shows looping vs one-shot animations.

### Key Concepts

- `AnimationElement` wraps any element and animates a property over time
- Easing functions control the interpolation curve
- Animations can be one-shot or looping
- Animation state is managed by the framework (not manual frame counting)

## View Layer (src/bin/animation_test.rs)

- Grid of colored boxes, each labeled with its easing function name
- All animate from left to right simultaneously on button press
- Second row: opacity fade-in with different durations
- Third row: size animation (grow/shrink)
- Toggle for loop vs one-shot mode

### Easing Functions to Demonstrate

- `linear`
- `ease_in` (slow start)
- `ease_out` (slow end)
- `ease_in_out` (slow both)
- `bounce` (if available)
- `spring` (if available)

## TDD Tests

### Animation construction (3)
1. AnimationElement wraps a child element without panic
2. Duration can be set
3. Easing function can be specified

### Animation behavior (3)
4. Animation progresses over time (t=0 → start value, t=1 → end value)
5. Looping animation resets after completion
6. One-shot animation stays at end value after completion

## Documentation (docs/gpui-usage/animation.md)

### Sections
1. **What it is** — element wrapper that interpolates properties over time with configurable easing
2. **Preconditions** — `use gpui::{AnimationElement, Animation, easing}` (verify exact imports from Zed source); element must be wrapped before adding to tree
3. **Signatures** — `AnimationElement::new(element, animation)`, `Animation::new(duration).with_easing(easing::ease_in_out)`, loop/one-shot config
4. **Relevant traits** — `IntoElement`
5. **Usage examples** — position animation, opacity animation, size animation, looping animation
6. **Post-conditions** — animations are cleaned up when element is removed from tree; looping animations run until element is dropped
7. **Testing** — use `cx.simulate_timer()` or `cx.advance_clock()` to test animation progress in headless tests
8. **Surprises** — animation rerenders every frame (performance cost); no built-in keyframe or sequence support; easing functions may differ from CSS equivalents; animation state resets if element is removed and re-added to tree

**Note:** Reference Zed's `crates/gpui/examples/animation.rs` for API patterns — the animation API surface should be verified against current Zed source before implementation.
