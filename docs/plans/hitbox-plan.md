# Hitbox

## Goal

Document `Hitbox`, `HitboxId`, and custom hit detection for non-rectangular or overlapping regions.

## Design

A canvas with non-rectangular clickable regions: circles, triangles, and overlapping shapes. Clicking a region highlights it and shows its ID. Demonstrates hitbox priority for overlapping regions.

### Key Concepts

- `Hitbox` defines a clickable region registered during prepaint
- `HitboxId` identifies a specific hitbox for event routing
- Hitboxes participate in hit testing — mouse events check hitbox membership
- Overlapping hitboxes have priority based on registration order (later = higher)
- Hitboxes work in canvas elements where div-based click handlers don't apply

## View Layer (src/bin/hitbox_test.rs)

- Canvas with several overlapping colored circles
- Click detection via hitbox — clicked circle highlights
- Status bar shows which HitboxId was hit
- Overlapping region demonstrates priority (top-most wins)
- Non-rectangular hit testing (circular hitbox within rectangular bounds — may need custom point-in-shape check in click handler)

### Integration with Existing draw_test

The existing `draw_test` already uses hitbox-like patterns for shape selection. This plan creates a focused standalone example to isolate the hitbox API.

## TDD Tests

### Hitbox creation (3)
1. Hitbox created with bounds returns valid HitboxId
2. Multiple hitboxes return unique HitboxIds
3. Hitbox is_hovered reports correctly for point inside bounds

### Hit testing (3)
4. Point inside hitbox registers as hit
5. Point outside hitbox does not register
6. Overlapping hitboxes — later registered hitbox takes priority

### Mouse events (2)
7. on_mouse_down with hitbox ID fires for correct region
8. on_mouse_down does not fire for wrong hitbox

## Documentation (docs/gpui-usage/hitbox.md)

### Sections
1. **What it is** — custom hit detection regions for canvas and non-standard elements
2. **Preconditions** — `use gpui::{Hitbox, HitboxId}`; hitboxes must be created during prepaint phase in canvas element; hitbox bounds are in window coordinates
3. **Signatures** — `window.insert_hitbox(bounds, opaque)`, `Hitbox { id, bounds, opaque }`, `hitbox.is_hovered(window)`, mouse handlers with hitbox checking
4. **Relevant types** — `Hitbox`, `HitboxId`
5. **Usage examples** — canvas hitbox, overlapping regions, click detection
6. **Post-conditions** — hitboxes are recreated each prepaint (not persistent); no cleanup needed
7. **Testing** — hit testing logic is testable via point-in-bounds checks; integration testing via simulated clicks
8. **Surprises** — hitboxes are RECTANGULAR only (non-rectangular shapes need manual point-in-shape logic in the handler); hitboxes must be recreated every prepaint (they're not retained); opaque hitboxes block hits to hitboxes behind them; hitbox coordinates are in window space not element-local space; insertion order determines priority (not z-index)
