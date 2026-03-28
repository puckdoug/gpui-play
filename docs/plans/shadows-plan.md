# Shadows

## Goal

Document `window.paint_shadows()`, `BoxShadow`, and style helpers `.shadow_sm()`, `.shadow_md()`, `.shadow_lg()`.

## Design

A view showing cards with different shadow configurations: style presets (sm, md, lg), custom shadows with offset/blur/spread/color, inner shadows, and `paint_shadows()` for canvas use.

### Key Concepts

- Style-based shadows: `.shadow_sm()`, `.shadow_md()`, `.shadow_lg()` — convenience presets
- `BoxShadow` struct: offset, blur, spread, color, inset
- `window.paint_shadows()` — low-level API for painting shadows in canvas/custom elements
- Multiple shadows can be combined on one element

## View Layer (src/bin/shadow_test.rs)

- Row 1: cards with `.shadow_sm()`, `.shadow_md()`, `.shadow_lg()` presets
- Row 2: custom shadows — large offset, colored shadow, zero-blur (hard shadow)
- Row 3: inner shadow (inset) vs outer shadow
- Row 4: canvas element using `window.paint_shadows()` directly
- Each card labeled with its shadow configuration

## TDD Tests

### Style shadows (3)
1. shadow_sm applies a BoxShadow to the element
2. shadow_md applies a larger BoxShadow
3. shadow_lg applies the largest BoxShadow

### Custom shadows (3)
4. BoxShadow with custom offset positions shadow correctly
5. BoxShadow with zero blur creates hard shadow
6. BoxShadow with inset creates inner shadow

## Documentation (docs/gpui-usage/shadows.md)

### Sections
1. **What it is** — drop shadows and inner shadows for elements and custom painting
2. **Preconditions** — `use gpui::BoxShadow` for custom; style helpers available on any `Styled` element; `window.paint_shadows()` requires paint callback context
3. **Signatures** — `.shadow_sm()`, `.shadow_md()`, `.shadow_lg()`, `.shadow(vec![BoxShadow { ... }])`, `window.paint_shadows(bounds, corner_radii, shadows)`
4. **Relevant types** — `BoxShadow`, `Pixels`
5. **Usage examples** — preset shadows, custom shadow, inset shadow, paint_shadows in canvas
6. **Post-conditions** — no cleanup; shadows repainted each frame
7. **Testing** — element construction testable; visual verification for correctness
8. **Surprises** — shadows render outside element bounds (need parent overflow visible); shadow blur is Gaussian (not box); multiple shadows stack (order matters); paint_shadows requires bounds that match the element; shadow color alpha affects intensity; no text shadows

**Note:** Reference Zed's `crates/gpui/examples/shadow.rs`.
