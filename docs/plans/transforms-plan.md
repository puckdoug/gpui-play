# Transforms

## Goal

Document `TransformationMatrix` for 2D transforms (rotate, scale, skew, translate).

## Design

A view showing elements with different transformations applied. Demonstrate individual transforms, composed transforms, and the effect of transform origin.

### Key Concepts

- `TransformationMatrix` is a 2D affine transformation matrix
- Transforms can be composed (multiply matrices)
- Transform origin affects the center of rotation/scale
- Transforms apply to the element and all its children

## View Layer (src/bin/transform_test.rs)

- Grid of colored boxes, each with a different transform:
  - Identity (reference)
  - Rotate 45°
  - Scale 1.5x
  - Scale 0.5x
  - Skew X 15°
  - Translate (20px, 10px)
  - Rotate + Scale (composed)
- Each box labeled with its transform
- Interactive: slider or buttons to adjust rotation angle live

## TDD Tests

### Matrix construction (3)
1. Identity matrix produces no change
2. Rotation matrix rotates point correctly
3. Scale matrix scales point correctly

### Composition (2)
4. Rotate then scale produces correct combined matrix
5. Order matters: rotate-then-scale != scale-then-rotate

### Application (2)
6. Transform applied to element affects rendered position
7. Transform applies to children

## Documentation (docs/gpui-usage/transforms.md)

### Sections
1. **What it is** — 2D affine transformations for elements (rotate, scale, skew, translate)
2. **Preconditions** — `use gpui::TransformationMatrix`; element must have defined bounds
3. **Signatures** — `TransformationMatrix::rotate(angle)`, `::scale(sx, sy)`, `::translate(tx, ty)`, matrix multiplication for composition
4. **Relevant types** — `TransformationMatrix`
5. **Usage examples** — rotation, scaling, composed transform, interactive transform
6. **Post-conditions** — transforms are visual only; hit testing may or may not account for transforms (verify); layout bounds unaffected by transform
7. **Testing** — matrix math is pure and testable; visual transform testing requires manual verification
8. **Surprises** — transforms do not affect layout (element still occupies untransformed space); transform origin may default to top-left not center; hit testing on transformed elements may be broken; composition order matters (right-to-left multiplication); no 3D transforms (perspective, rotateX/Y)
