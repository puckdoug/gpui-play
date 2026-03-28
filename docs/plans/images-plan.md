# Images (Img, ImageCache, ObjectFit)

## Goal

Document and demonstrate image rendering: `Img` element, `ImageSource` variants, `ObjectFit` modes, and `ImageCache`.

## Design

A gallery view showing the same image rendered with each `ObjectFit` mode (Contain, Cover, Fill, ScaleDown, None) in labeled containers. Includes loading from embedded bytes and from file path. Shows `ImageCache` usage for deduplication.

### Data Model

```rust
struct ImageGallery {
    images: Vec<ImageEntry>,
}

struct ImageEntry {
    source: ImageSource,
    label: String,
}
```

### ObjectFit Modes to Demonstrate

- `Contain` — scales to fit within bounds, preserving aspect ratio (may letterbox)
- `Cover` — scales to cover bounds, preserving aspect ratio (may crop)
- `Fill` — stretches to fill bounds exactly (may distort)
- `ScaleDown` — like Contain but never scales up
- `None` — renders at natural size, may overflow

## State Layer

Minimal — image loading is mostly framework-managed. Focus on documenting `ImageSource` variants:
- `ImageSource::File(PathBuf)` — load from disk
- `ImageSource::Data(SharedImage)` — embedded/in-memory bytes
- `ImageSource::Surface(Surface)` — pixel buffer (covered in screen-capture plan)

## View Layer (src/bin/image_test.rs)

- Grid layout showing 5 copies of same image, one per ObjectFit mode
- Each cell labeled with the ObjectFit name
- Second row: different ImageSource types (file vs embedded)
- Border around each container to make fit behavior visible

## Assets

- Include a small test image (e.g., `assets/test.png`) — a non-square image to make ObjectFit differences visible
- Embed one image via `include_bytes!()` for the Data source demo

## TDD Tests

### Image element (4)
1. Img element creates without panic
2. Each ObjectFit variant is accepted
3. ImageSource::Data with embedded bytes loads successfully
4. ImageSource::File with valid path loads successfully

### ImageCache (2)
5. Same image loaded twice returns cached version
6. Cache can be cleared

## Documentation (docs/gpui-usage/images.md)

### Sections
1. **What it is** — GPU-rendered image element supporting PNG, JPEG, GIF, WebP with async loading
2. **Preconditions** — `use gpui::{img, ImageSource, ObjectFit}`; image files accessible at path or embedded via `include_bytes!`
3. **Signatures** — `img(source)`, `.object_fit(ObjectFit::Cover)`, `.size()`, `ImageSource` variants
4. **Relevant traits** — `IntoElement` (Img implements it)
5. **Usage examples** — file image, embedded image, each ObjectFit mode
6. **Post-conditions** — images loaded async, may flash on first render; ImageCache retains decoded images in memory
7. **Testing** — limited without visual verification; can test element construction and source types
8. **Surprises** — images load asynchronously (blank on first frame); GIF animation is automatic; ObjectFit::None can overflow container; large images consume GPU memory; no built-in lazy loading
