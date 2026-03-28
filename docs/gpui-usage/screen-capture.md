# Screen Capture & Surface

**Components:** [`ScreenCaptureSource`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs), [`ScreenCaptureStream`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform.rs), [`Surface`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/surface.rs)

## What is the component and what it does

GPUI supports screen/window capture on macOS via the `ScreenCaptureSource` and `ScreenCaptureStream` traits. Captured frames are rendered using the `Surface` element, which displays CoreVideo pixel buffers.

This is the most complex platform feature — it requires OS permissions, produces live video frames, and consumes significant CPU/GPU resources.

## Preconditions for use

```rust
// Types are traits, not concrete structs
use gpui::App; // cx.screen_capture_sources()
```

- **macOS only** — requires screen recording permission
- Permission must be granted in System Preferences > Privacy & Security > Screen Recording
- The permission prompt shows only once — subsequent denials require manual settings change
- `Surface` element renders CoreVideo pixel buffers (macOS only)
- App may need entitlements for distribution

## Signature for usage

### Enumerate capture sources

```rust
cx.screen_capture_sources() -> oneshot::Receiver<Result<Vec<Rc<dyn ScreenCaptureSource>>>>
```

### ScreenCaptureSource trait

```rust
pub trait ScreenCaptureSource {
    fn metadata(&self) -> Result<SourceMetadata>;
    fn stream(
        &self,
        foreground_executor: &ForegroundExecutor,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<Result<Box<dyn ScreenCaptureStream>>>;
}
```

### SourceMetadata

```rust
pub struct SourceMetadata {
    pub id: u64,
    pub label: Option<SharedString>,     // Display/window name
    pub is_main: Option<bool>,           // Is primary display
    pub resolution: Size<DevicePixels>,  // Capture resolution
}
```

### ScreenCaptureStream trait

```rust
pub trait ScreenCaptureStream {
    fn metadata(&self) -> Result<SourceMetadata>;
}
```

### ScreenCaptureFrame

```rust
pub struct ScreenCaptureFrame(pub PlatformScreenCaptureFrame);
```

## Usage and examples

### Enumerate and start capture (conceptual)

```rust
// 1. Request capture sources (async)
let receiver = cx.screen_capture_sources();

// 2. In spawned task, await sources
let sources = receiver.await?;

// 3. Get metadata for each source
for source in &sources {
    let metadata = source.metadata()?;
    println!("{}: {:?}", metadata.id, metadata.label);
}

// 4. Start stream from a source
let stream_receiver = source.stream(
    &foreground_executor,
    Box::new(|frame| {
        // Handle each captured frame
        // Render frame in Surface element
    }),
);
let stream = stream_receiver.await?;
```

### Surface element (for rendering frames)

The `Surface` element renders CoreVideo pixel buffers. It's used to display captured frames or video content.

## Post-conditions / destruction requirements

- **Stream must be stopped** explicitly or on drop
- Surface holds GPU resources — release when no longer needed
- Permission persists across launches but can be revoked in System Preferences
- High CPU/GPU usage during active capture — stop when not needed

## Testing

Screen capture requires OS permissions. Tests verify API availability only:

```rust
#[gpui::test]
fn test_api_exists(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let _receiver = cx.screen_capture_sources();
    });
}
```

Run tests: `cargo test --test screen_capture_test`

## Surprises, Anti-patterns, and Bugs

### Permission prompt shows only once

macOS shows the screen recording permission dialog only the first time. If denied, the user must manually enable it in System Preferences. There is no way to re-trigger the prompt programmatically.

### Capturing own windows may not work

The capturing app's own windows may appear blank or be excluded from the capture, depending on macOS version and window server behavior.

### High resource usage

Screen capture is expensive — each frame is a full-resolution bitmap. Expect significant CPU and GPU usage. Limit frame rate or resolution if possible.

### Types are traits, not structs

`ScreenCaptureSource` and `ScreenCaptureStream` are traits, not concrete types. You receive `Rc<dyn ScreenCaptureSource>` and `Box<dyn ScreenCaptureStream>`. This means no direct construction — only obtained from the framework.

### No built-in recording to file

Captured frames must be encoded manually if you want to save a recording. GPUI provides frame-by-frame access, not a recording API.

### Surface is macOS only

The `Surface` element uses CoreVideo for pixel buffer rendering. It is not available on other platforms.

### DRM-protected content may show blank

Capturing windows from apps that use DRM (e.g., some streaming services) may produce blank frames.
