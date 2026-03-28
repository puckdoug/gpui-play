# SVG Rendering

## Goal

Document and demonstrate `Svg` element with `Transformation` (rotate, scale, translate) and color tinting.

## Design

A view displaying several SVG icons with different transformations applied. Show identity, rotated, scaled, translated, and composed transformations. Demonstrate color tinting via the `.text_color()` style (SVGs inherit text color for fill).

### Key Concepts

- SVG files must be simple path-based SVGs (no embedded raster, no CSS, no JavaScript)
- GPUI renders SVGs as vector paths, not as embedded browser content
- Color is controlled via the parent element's `text_color()` — the SVG inherits it
- `Transformation` applies rotate, scale, translate to the rendered SVG

## View Layer (src/bin/svg_test.rs)

- Row of SVG icons at default rendering
- Row with rotations (0°, 45°, 90°, 180°)
- Row with scale transformations (0.5x, 1x, 2x)
- Row with different colors via `text_color()`
- Row with composed transform (rotate + scale)

## Assets

- Include 2-3 simple SVG icons in `assets/` (e.g., arrow, circle, star)
- Keep SVGs minimal — single path elements

## TDD Tests

### SVG element (3)
1. Svg element creates from valid SVG path without panic
2. Transformation can be applied (rotate)
3. Transformation can be composed (rotate + scale)

### Color (1)
4. SVG renders with inherited text_color (verify element tree construction)

## Documentation (docs/gpui-usage/svg.md)

### Sections
1. **What it is** — vector SVG rendering element with transform support
2. **Preconditions** — `use gpui::svg`; SVG file must be path-based (no raster, no CSS); file path relative to project root or embedded
3. **Signatures** — `svg().path("path/to/icon.svg")`, `.transformation(Transformation::rotate(angle))`, `.size()`
4. **Relevant traits** — `IntoElement`
5. **Usage examples** — basic SVG, rotated, scaled, colored
6. **Post-conditions** — SVGs loaded and cached; no cleanup needed
7. **Testing** — element construction testable; visual correctness requires manual verification
8. **Surprises** — only simple SVGs supported (no gradients, no filters, no text elements in SVG); color controlled via text_color not fill attribute; transformation origin may not be center of element; complex SVGs silently render incorrectly or not at all
