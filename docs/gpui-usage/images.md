# Images (Img, ObjectFit, ImageCache)

**Components:** [`Img`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/img.rs), [`ObjectFit`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/style.rs), [`ImageSource`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/img.rs), [`ImageCache`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/image_cache.rs)

## What is the component and what it does

The `Img` element renders images (PNG, JPEG, GIF, WebP, SVG, TIFF, BMP, ICO, and more) with GPU acceleration. Images load asynchronously and support multiple source types: file paths, embedded assets, URLs, raw bytes, and custom loaders. `ObjectFit` controls how the image scales within its bounds, matching CSS `object-fit` behavior.

`ImageCache` manages decoded image caching and deduplication. A built-in `RetainAllImageCache` is available, or you can implement the `ImageCache` trait for custom caching strategies.

## Preconditions for use

```rust
use gpui::{img, ObjectFit, Styled, StyledImage};
```

- `StyledImage` trait must be in scope for `.object_fit()`, `.grayscale()`, `.with_fallback()`, `.with_loading()`
- `Styled` trait must be in scope for `.size_full()`, `.size_8()`, etc.
- Images load asynchronously — the first render frame may show blank or the loading placeholder

## Signature for usage

### Creating an image element

```rust
// From a path string (asset or file)
img("path/to/image.png")

// From an Arc<Path>
img(Arc::new(PathBuf::from("/absolute/path.png")))

// From raw bytes with format
let image = Arc::new(Image::from_bytes(ImageFormat::Png, bytes));
img(image)

// From a custom loader function
img(|window: &mut Window, cx: &mut App| {
    // Custom loading logic returning Option<Result<Arc<RenderImage>, ImageCacheError>>
    None
})
```

### ImageSource variants

```rust
pub enum ImageSource {
    Resource(Resource),                   // URI, file path, or embedded asset
    Render(Arc<RenderImage>),             // Pre-decoded image frames
    Image(Arc<Image>),                    // Raw bytes with format
    Custom(Arc<dyn Fn(...) -> ...>),      // Custom loader function
}
```

### Resource variants

```rust
pub enum Resource {
    Uri(SharedUri),          // Remote URL
    Path(Arc<Path>),         // Local file path
    Embedded(SharedString),  // Embedded in binary via AssetSource
}
```

### ObjectFit modes

```rust
ObjectFit::Fill       // Stretch to fill bounds (ignores aspect ratio)
ObjectFit::Contain    // Scale to fit within bounds (letterbox)
ObjectFit::Cover      // Scale to cover bounds (may crop)
ObjectFit::ScaleDown  // Like Contain but never scales up
ObjectFit::None       // Render at natural size (may overflow)
```

### Styling methods (StyledImage trait)

```rust
img("photo.jpg")
    .object_fit(ObjectFit::Cover)
    .grayscale(true)
    .with_fallback(|| div().child("Error").into_any_element())
    .with_loading(|| div().child("Loading...").into_any_element())
    .size_full()
```

### ImageCache

```rust
// Create a cache entity
let cache = RetainAllImageCache::new(cx);

// Use on a specific image
img("url").image_cache(&cache)

// Clear cache
cache.update(cx, |cache, cx| cache.clear(window, cx));
```

## Relevant Traits

| Trait | Purpose |
|-------|---------|
| `StyledImage` | Provides `.object_fit()`, `.grayscale()`, `.with_fallback()`, `.with_loading()` |
| `Styled` | Provides `.size_full()`, `.size_8()`, etc. for sizing |
| `ImageCache` | Trait for implementing custom image caches |
| `IntoElement` | `Img` implements this — usable as a child element |

## Usage and examples

### Basic image with object fit

```rust
impl Render for ImageView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            img("assets/photo.png")
                .object_fit(ObjectFit::Contain)
                .size_full(),
        )
    }
}
```

### All ObjectFit modes side by side

```rust
div()
    .child(img("photo.png").object_fit(ObjectFit::Fill))
    .child(img("photo.png").object_fit(ObjectFit::Contain))
    .child(img("photo.png").object_fit(ObjectFit::Cover))
    .child(img("photo.png").object_fit(ObjectFit::ScaleDown))
    .child(img("photo.png").object_fit(ObjectFit::None))
```

### Fallback and loading states

```rust
img("https://example.com/slow-image.jpg")
    .with_loading(|| div().child("Loading...").into_any_element())
    .with_fallback(|| div().child("Failed to load").into_any_element())
    .size_full()
```

## Post-conditions / destruction requirements

- Images are loaded asynchronously — the element may render blank on the first frame
- `ImageCache` retains decoded images in memory; call `.clear()` to free them
- GIF animations play automatically — no explicit start/stop
- No explicit cleanup needed for `Img` elements; they are released when removed from the tree

## Testing

Visual image rendering cannot be verified in headless tests. Tests verify element construction and API surface:

```rust
#[gpui::test]
fn test_img_element(cx: &mut TestAppContext) {
    struct ImageView;
    impl Render for ImageView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(img("test.png").object_fit(ObjectFit::Cover).size_full())
        }
    }
    let _window = cx.add_window(|_window, _cx| ImageView);
}
```

Run tests: `cargo test --test images_test`

## Surprises, Anti-patterns, and Bugs

### Images load asynchronously

The first render frame shows blank (or the `.with_loading()` placeholder). This is expected — image decoding happens off the main thread.

### `StyledImage` trait must be in scope

Without `use gpui::StyledImage`, methods like `.object_fit()`, `.grayscale()`, `.with_fallback()`, and `.with_loading()` are not available. The compiler error says "method not found" — the fix is importing the trait.

### ObjectFit::None can overflow

If the image is larger than its container and `ObjectFit::None` is used, the image renders at its natural size and overflows. The parent must handle clipping.

### Supported formats are extensive

`Img::extensions()` returns: avif, jpg, jpeg, png, gif, webp, tif, tiff, tga, dds, bmp, ico, hdr, exr, pbm, pam, ppm, pgm, ff, farbfeld, qoi, svg. Yes, SVG is supported through the image element too.

### No built-in lazy loading

All images in the element tree begin loading immediately. There is no intersection-observer-style lazy loading. For large lists of images, use a virtualized list to limit the number of simultaneously loading images.

### Large images consume GPU memory

Each decoded image is uploaded to the GPU as a texture. Very large images or many images can exhaust GPU memory. The `ImageCache` helps with deduplication but does not limit memory usage.
