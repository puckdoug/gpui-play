# Display & Appearance

## Goal

Document `displays()`, `primary_display()`, `find_display()`, and `window_appearance()` (dark/light mode).

## Design

A view that shows connected display information and adapts its styling to the current system appearance (dark/light mode).

### Key Concepts

- `cx.displays()` — returns all connected displays with resolution, scale factor, bounds
- `cx.primary_display()` — the main display
- `cx.find_display(id)` — find display by ID
- `cx.window_appearance()` — returns `WindowAppearance` (Light, VibrantLight, Dark, VibrantDark)
- Appearance can change at runtime (user toggles system dark mode)

## View Layer (src/bin/appearance_test.rs)

- Display info panel: list all displays with resolution, scale, and bounds
- Primary display highlighted
- Appearance section: current mode displayed, background/text colors adapt
- Two-column comparison: forced light vs forced dark styling (simulated)

## TDD Tests

### Display (3)
1. displays() returns at least one display
2. primary_display() returns a valid display
3. Display has resolution and scale factor

### Appearance (2)
4. window_appearance() returns a valid variant
5. Appearance value can be matched to set colors

## Documentation (docs/gpui-usage/display-appearance.md)

### Sections
1. **What it is** — query connected displays and system appearance for adaptive UI
2. **Preconditions** — `use gpui::WindowAppearance`; display queries via `cx` (App context)
3. **Signatures** — `cx.displays()`, `cx.primary_display()`, `cx.find_display(id)`, `cx.window_appearance()`
4. **Relevant types** — `Display`, `DisplayId`, `WindowAppearance`
5. **Usage examples** — list displays, adaptive dark/light theming
6. **Post-conditions** — display info is a snapshot (may change if monitors connected/disconnected); no listener for display changes
7. **Testing** — display queries work in test context; appearance may default to a specific value in tests
8. **Surprises** — no callback for display connect/disconnect (must poll); VibrantLight/VibrantDark may not behave differently from Light/Dark in practice; scale factor is important for pixel-perfect rendering; display bounds are in logical pixels not physical; window_appearance may not update immediately when user toggles system appearance
