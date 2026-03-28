use gpui::{point, px, size, ScaledPixels, TransformationMatrix};

// TransformationMatrix is the low-level 2D affine transform used internally.
// Methods are on instances (self), not associated functions.
// NOT available as a style method on divs — only used with SVG Transformation
// and during canvas painting.

#[test]
fn test_identity_matrix() {
    let m = TransformationMatrix::unit();
    let p = point(px(10.0), px(20.0));
    let result = m.apply(p);
    assert_eq!(result.x, px(10.0));
    assert_eq!(result.y, px(20.0));
}

#[test]
fn test_translation_matrix() {
    let m = TransformationMatrix::unit()
        .translate(point(ScaledPixels(5.0), ScaledPixels(10.0)));
    let p = point(px(0.0), px(0.0));
    let result = m.apply(p);
    assert_eq!(result.x, px(5.0));
    assert_eq!(result.y, px(10.0));
}

#[test]
fn test_scale_matrix() {
    let m = TransformationMatrix::unit().scale(size(2.0, 3.0));
    let p = point(px(10.0), px(10.0));
    let result = m.apply(p);
    assert_eq!(result.x, px(20.0));
    assert_eq!(result.y, px(30.0));
}

#[test]
fn test_compose_translate_then_scale() {
    let translated = TransformationMatrix::unit()
        .translate(point(ScaledPixels(10.0), ScaledPixels(0.0)));
    let scaled = TransformationMatrix::unit().scale(size(2.0, 2.0));
    let composed = translated.compose(scaled);

    let p = point(px(5.0), px(5.0));
    let result = composed.apply(p);
    // compose(other) applies other first, then self
    // scale(5,5) -> (10,10), then translate -> (20,10)
    assert_eq!(result.x, px(20.0));
    assert_eq!(result.y, px(10.0));
}

#[test]
fn test_compose_order_matters() {
    let translated = TransformationMatrix::unit()
        .translate(point(ScaledPixels(10.0), ScaledPixels(0.0)));
    let scaled = TransformationMatrix::unit().scale(size(2.0, 2.0));

    let a = translated.compose(scaled);
    let b = scaled.compose(translated);

    let p = point(px(5.0), px(5.0));
    let ra = a.apply(p);
    let rb = b.apply(p);

    assert_ne!(ra.x, rb.x);
}

#[test]
fn test_rotation_90_degrees() {
    use std::f32::consts::FRAC_PI_2;
    let m = TransformationMatrix::unit().rotate(gpui::Radians(FRAC_PI_2));
    let p = point(px(10.0), px(0.0));
    let result = m.apply(p);
    // Rotating (10, 0) by 90° gives approximately (0, 10)
    assert!((result.x - px(0.0)).abs() < px(0.01));
    assert!((result.y - px(10.0)).abs() < px(0.01));
}
