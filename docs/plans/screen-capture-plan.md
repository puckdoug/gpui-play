# Screen Capture & Surface

## Goal

Document `ScreenCaptureSource`, `ScreenCaptureStream`, and `Surface` (CoreVideo pixel buffer rendering).

## Design

A view that captures screen content and displays it in a `Surface` element. Shows the permission flow, source selection, and live rendering.

### Key Concepts

- `ScreenCaptureSource` — identifies a capturable screen or window
- `ScreenCaptureStream` — ongoing capture producing frames
- `Surface` — element that renders a CoreVideo pixel buffer (the capture output)
- macOS only — requires screen recording permission
- This is the most complex platform feature

### Permission Flow

1. Check screen recording permission
2. Request permission if not granted (opens System Preferences)
3. Enumerate capture sources (screens, windows)
4. Create capture stream from selected source
5. Receive frames and render in Surface element

## View Layer (src/bin/capture_test.rs)

- "Request Permission" button (if not already granted)
- Source picker: list available screens/windows
- "Start Capture" button → begins stream
- Surface element displaying the captured frames
- "Stop Capture" button → ends stream
- Frame counter / FPS display

## TDD Tests

### API availability (2)
1. ScreenCaptureSource type exists and can be referenced
2. Surface element can be constructed

### Note on testability
Screen capture requires OS permissions and live screen content. Tests verify API surface only. Full behavior requires manual testing with permissions granted.

### Stream lifecycle (2)
3. Capture stream can be started (given permissions)
4. Capture stream can be stopped and resources released

## Documentation (docs/gpui-usage/screen-capture.md)

### Sections
1. **What it is** — screen/window capture with live pixel buffer rendering
2. **Preconditions** — macOS only; screen recording permission required; `use gpui::{ScreenCaptureSource, ScreenCaptureStream, Surface}`; app may need entitlements for distribution
3. **Signatures** — source enumeration, stream creation, Surface element construction, frame callback
4. **Relevant types** — `ScreenCaptureSource`, `ScreenCaptureStream`, `Surface`
5. **Usage examples** — permission check, source enumeration, capture loop, Surface display
6. **Post-conditions** — stream MUST be stopped explicitly or on drop; Surface holds GPU resources; permission persists across launches but can be revoked in System Preferences
7. **Testing** — API surface testable; behavior requires manual testing with permissions
8. **Surprises** — permission prompt only shows once (must be granted in System Preferences after denial); capture may not include the capturing app's own windows; high CPU/GPU usage during capture; frame rate may be limited by system; Surface is macOS-only (CoreVideo); capture of other apps' windows may show blank if they use DRM; no built-in recording to file (frames must be encoded manually)
