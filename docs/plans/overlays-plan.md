# Overlays (Anchored, Deferred, Tooltips)

## Goal

Document `Anchored`, `Deferred`, and tooltip support (`.tooltip()`, `TooltipId`).

## Design

A view with buttons that open floating UI: a popover menu (Anchored + Deferred), a tooltip on hover, and a nested popover demonstrating layering.

### Key Concepts

- `Deferred` renders its children after all ancestors — ensures overlays paint on top
- `Anchored` positions content relative to an anchor point, adjusting to stay within window bounds
- Tooltips are a specialized overlay pattern using `.tooltip()` on any element
- Combining Deferred + Anchored is the standard pattern for popovers, dropdowns, context menus

## View Layer (src/bin/overlay_test.rs)

- Row of buttons:
  - "Open Popover" → shows Anchored popover below the button with menu items
  - "Nested" → popover with a button that opens a second popover
  - Hover targets with tooltips showing different content
- Popover near window edge → demonstrates Anchored boundary avoidance
- Click outside popover → dismiss (focus management)

### Rendering Stack

```
Deferred {
    Anchored(anchor_corner, fit_mode) {
        popover content (div with border, shadow, items)
    }
}
```

## TDD Tests

### Deferred (2)
1. Deferred element renders after its ancestors
2. Multiple Deferred elements render in correct order

### Anchored (3)
3. Anchored positions content relative to anchor point
4. Anchored adjusts position to avoid window boundaries
5. Anchored with different anchor corners (top-left, bottom-right, etc.)

### Tooltips (2)
6. Tooltip appears on hover after delay
7. Tooltip disappears on mouse leave

### Integration (2)
8. Popover opens on click, closes on outside click
9. Nested popover layers correctly over parent popover

## Documentation (docs/gpui-usage/overlays.md)

### Sections
1. **What it is** — floating UI system for popovers, dropdowns, tooltips, and context menus
2. **Preconditions** — `use gpui::{Anchored, Deferred, AnchorCorner}`; Deferred must wrap Anchored for correct paint order; parent must manage visibility state
3. **Signatures** — `deferred(child)`, `anchored().child(content).anchor(corner)`, `.tooltip(|cx| view)`, `TooltipId`
4. **Relevant types** — `Anchored`, `Deferred`, `AnchorCorner`, `FitMode`
5. **Usage examples** — basic popover, tooltip, nested overlay, boundary avoidance
6. **Post-conditions** — overlays removed from tree when parent conditionally hides them; no explicit cleanup
7. **Testing** — test visibility state; anchored positioning testable via layout bounds
8. **Surprises** — Anchored without Deferred renders behind siblings; focus management must be handled manually for dismiss-on-outside-click; tooltip delay is framework-controlled; nested Anchored elements may fight over boundary adjustment; Deferred has performance cost (extra paint pass)

**Note:** Reference Zed's `crates/gpui/examples/popover.rs`.
