# Platform Utilities

## Goal

Document miscellaneous platform APIs: keychain, URL schemes, keyboard layout, thermal state, dock menu, recent documents, and system open/reveal.

## Design

A dashboard view with sections for each API, each with a button or display showing the API in action.

### APIs Covered

| API | Description | Category |
|-----|-------------|----------|
| `write_credentials()` / `read_credentials()` | macOS Keychain access | Security |
| `register_url_scheme()` / `on_open_urls()` | Custom URL scheme handling | Integration |
| `keyboard_layout()` / `on_keyboard_layout_change()` | Current keyboard layout | Input |
| `thermal_state()` / `on_thermal_state_change()` | CPU thermal throttling | System |
| `set_dock_menu()` | macOS dock right-click menu | UI |
| `add_recent_document()` | Recent documents in dock menu | UI |
| `open_with_system()` | Open file with default app | Integration |
| `reveal_path()` | Show file in Finder | Integration |

## View Layer (src/bin/platform_test.rs)

- **Keychain section:** text field for key/value, "Save" and "Load" buttons
- **URL section:** display registered scheme, log of opened URLs
- **Keyboard section:** current layout name, change listener
- **Thermal section:** current thermal state (nominal, fair, serious, critical)
- **Dock menu section:** button to set custom dock menu items
- **Open/Reveal section:** buttons to open a file or reveal in Finder

## TDD Tests

### Keychain (2)
1. write_credentials then read_credentials returns same value
2. read_credentials for nonexistent key returns None

### Keyboard layout (1)
3. keyboard_layout() returns non-empty string

### Thermal (1)
4. thermal_state() returns valid variant

### Open/Reveal (1)
5. open_with_system and reveal_path functions exist and accept PathBuf

### Note on testability
Many platform APIs require app entitlements, specific hardware, or OS state. Tests verify API availability and basic types. Full behavior is manual testing.

## Documentation (docs/gpui-usage/platform-utils.md)

### Sections
1. **What it is** — collection of macOS/platform integration APIs
2. **Preconditions** — most require `cx: &mut App` or `cx: &mut Window`; keychain may require entitlements for signed apps; URL schemes require registration before use
3. **Signatures** — each API with its full signature
4. **Relevant types** — `ThermalState`, `WindowAppearance`, etc.
5. **Usage examples** — one example per API
6. **Post-conditions** — keychain entries persist across app launches; URL scheme registration persists; dock menu persists until changed; recent documents managed by OS
7. **Testing** — minimal automated testing possible; verify API existence and types
8. **Surprises** — keychain access may prompt for permission on first use; URL scheme registration may conflict with other apps; thermal_state is macOS-specific; dock menu only visible when app is in dock; on_open_urls only fires if app is already running (cold launch URLs handled differently); reveal_path is Finder-specific (no Linux equivalent)
