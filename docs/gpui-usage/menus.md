# Menus

**Components:** [`Menu`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform/app_menu.rs#L4), [`MenuItem`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform/app_menu.rs#L76), [`OwnedMenu`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform/app_menu.rs#L238), [`OwnedMenuItem`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform/app_menu.rs#L250), [`OsAction`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform/app_menu.rs#L311), [`SystemMenuType`](https://github.com/zed-industries/zed/blob/main/crates/gpui/src/platform/app_menu.rs#L70)

## What is the component and what it does

GPUI's menu system creates native application menu bars on macOS (and platform equivalents elsewhere). Menus are defined as a tree of `Menu` and `MenuItem` structs, passed to `cx.set_menus()` on the `App` context. The first menu in the list becomes the application menu (the bold-named menu in the macOS menu bar).

Menu items can be:
- **Actions** — trigger a named action when clicked
- **Separators** — visual dividers between groups of items
- **Submenus** — nested menus containing more items
- **System menus** — OS-managed menus (e.g., macOS Services)

Items can be individually disabled (grayed out) or checked (with a checkmark).

## Signature for usage

### Menu

```rust
// Create a menu with a name
Menu::new(name: impl Into<SharedString>) -> Menu

// Set items on the menu
menu.items(items: impl IntoIterator<Item = MenuItem>) -> Menu

// Disable the entire menu
menu.disabled(disabled: bool) -> Menu

// Convert to OwnedMenu (for inspection/testing)
menu.owned() -> OwnedMenu
```

### MenuItem

```rust
// Action item — triggers an Action when clicked
MenuItem::action(name: impl Into<SharedString>, action: impl Action) -> MenuItem

// Action item with OS action hint (Cut, Copy, Paste, Undo, Redo, SelectAll)
MenuItem::os_action(name: impl Into<SharedString>, action: impl Action, os_action: OsAction) -> MenuItem

// Visual separator
MenuItem::separator() -> MenuItem

// Nested submenu
MenuItem::submenu(menu: Menu) -> MenuItem

// OS-managed submenu (e.g., Services)
MenuItem::os_submenu(name: impl Into<SharedString>, menu_type: SystemMenuType) -> MenuItem

// Builder methods on MenuItem
item.disabled(disabled: bool) -> MenuItem  // Gray out the item
item.checked(checked: bool) -> MenuItem    // Show a checkmark
item.is_disabled() -> bool
item.is_checked() -> bool

// Convert to OwnedMenuItem (for inspection/testing)
item.owned() -> OwnedMenuItem
```

### App context

```rust
// Register key bindings (call BEFORE set_menus for shortcut display)
cx.bind_keys(bindings: impl IntoIterator<Item = KeyBinding>)

// Set the application menu bar
cx.set_menus(menus: impl IntoIterator<Item = Menu>)

// Read back menus (returns None on test platform)
cx.get_menus() -> Option<Vec<OwnedMenu>>

// Register a global action handler (used for menu actions)
cx.on_action(listener: impl Fn(&A, &mut App) + 'static)
```

### KeyBinding

```rust
// Create a key binding: keystroke string, action, optional context predicate
KeyBinding::new(keystrokes: &str, action: impl Action, context: Option<&str>) -> KeyBinding

// Inspect a binding
binding.keystrokes() -> &[KeybindingKeystroke]  // the keystroke(s)
binding.action() -> &dyn Action                 // the bound action
```

Keystroke format: modifiers joined by `-`, followed by the key. Examples: `"cmd-q"`, `"cmd-shift-z"`, `"ctrl-c"`, `"alt-f4"`. Modifier names: `cmd`, `ctrl`, `alt`, `shift`, `fn`.

## Relevant Macros

### `actions!()`

Defines action types that menu items can trigger. Actions are unit structs that implement the `Action` trait.

```rust
actions!(my_module, [Quit, About, Undo, Redo, Cut, Copy, Paste]);
```

This generates structs like `pub struct Quit;` etc., each implementing `Action`. The first argument is a namespace module name, the second is the list of action names.

## Relevant Traits

### `Action`

All menu item actions must implement `Action`. The `actions!()` macro handles this automatically. Custom actions with data fields need to derive `Action` manually.

### `Global` (for menu state)

If your menu state needs to be shared (e.g., checked items reflecting app state), use `impl Global for YourState` and access it via `cx.global::<YourState>()` / `cx.global_mut::<YourState>()` within action handlers, then call `cx.set_menus()` again to refresh.

## Usage and examples

### Basic menu setup with keyboard shortcuts

```rust
use gpui::{actions, App, KeyBinding, Menu, MenuItem};

actions!(menu_test, [Quit, About, Undo, Redo, Cut, Copy, Paste, Search]);

pub fn key_bindings() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-c", Copy, None),
        KeyBinding::new("cmd-v", Paste, None),
        KeyBinding::new("cmd-z", Undo, None),
        KeyBinding::new("cmd-shift-z", Redo, None),
    ]
}

pub fn menus() -> Vec<Menu> {
    vec![
        Menu::new("MenuTest").items([
            MenuItem::action("About MenuTest", About).disabled(true),
        ]),
        Menu::new("File").items([
            MenuItem::action("Quit", Quit),
        ]),
        Menu::new("Edit").items([
            MenuItem::action("Undo", Undo).disabled(true),
            MenuItem::action("Redo", Redo).disabled(true),
            MenuItem::separator(),
            MenuItem::action("Cut", Cut).disabled(true),
            MenuItem::action("Copy", Copy).disabled(true),
            MenuItem::action("Paste", Paste).disabled(true),
        ]),
        Menu::new("Help").items([
            MenuItem::action("Search", Search).disabled(true),
        ]),
    ]
}

pub fn setup_menus(cx: &mut App) {
    // bind_keys MUST be called before set_menus for shortcuts to display
    cx.bind_keys(key_bindings());
    cx.set_menus(menus());
    cx.on_action(|_: &Quit, cx: &mut App| cx.quit());
}
```

macOS automatically displays the keyboard shortcut (e.g., ⌘Q) next to the menu item when a matching `KeyBinding` is registered for the same action. The shortcut symbol rendering (⌘ for Cmd, ⇧ for Shift, etc.) is handled by the native menu system.

### Testing menus

The test platform's `set_menus()` is a **no-op** and `get_menus()` always returns `None`. You cannot verify menus through the `App` context in tests.

Instead, extract your menu definition into a pure function returning `Vec<Menu>`, and test it using `.owned()` to convert to inspectable `OwnedMenu`/`OwnedMenuItem` types:

```rust
use gpui::{OwnedMenu, OwnedMenuItem};

fn owned_menus() -> Vec<OwnedMenu> {
    menus().into_iter().map(|m| m.owned()).collect()
}

#[test]
fn test_creates_four_menus() {
    let menus = owned_menus();
    assert_eq!(menus.len(), 4);
    assert_eq!(menus[0].name.as_ref(), "MenuTest");
}

#[test]
fn test_quit_is_enabled() {
    let menus = owned_menus();
    let file_menu = menus.iter().find(|m| m.name.as_ref() == "File").unwrap();
    for item in &file_menu.items {
        if let OwnedMenuItem::Action { name, disabled, .. } = item {
            if name == "Quit" {
                assert!(!disabled);
            }
        }
    }
}
```

### Testing keybindings

Like menus, the test platform's keymap is not publicly accessible. Extract keybindings into a pure function returning `Vec<KeyBinding>` and test directly:

```rust
use gpui::KeyBinding;

#[test]
fn test_keybindings_defined() {
    let bindings = key_bindings();

    let expected = vec![
        ("Quit", "cmd-q"),
        ("Copy", "cmd-c"),
        ("Paste", "cmd-v"),
        ("Undo", "cmd-z"),
        ("Redo", "cmd-shift-z"),
    ];

    for (action_name, expected_keys) in &expected {
        let found = bindings.iter().any(|b: &KeyBinding| {
            let keystrokes = b.keystrokes();
            keystrokes.len() == 1
                && keystrokes[0].unparse() == *expected_keys
                && format!("{:?}", b.action()).contains(action_name)
        });
        assert!(found, "expected keybinding '{}' for '{}'", expected_keys, action_name);
    }
}
```

`KeybindingKeystroke::unparse()` returns the canonical string form (e.g., `"cmd-shift-z"`). Use `format!("{:?}", binding.action())` to get the action's debug name for matching.

### Application name in macOS menu bar

On macOS, the application menu name (the bold text in the menu bar) is determined by the **binary/process name**, not by the name passed to `Menu::new()` for the first menu. The `Menu::new("MenuTest")` name is used as the menu's internal title, but macOS overrides the display with the executable name.

To control the application name in the menu bar, set the binary name in `Cargo.toml`:

```toml
[[bin]]
name = "MenuTest"
path = "src/bin/menu_test.rs"
```

This makes `cargo run --bin MenuTest` launch with the correct app menu name.

### OwnedMenu / OwnedMenuItem for inspection

`Menu` and `MenuItem` contain `Box<dyn Action>` which is not `Clone`. The `Owned` variants exist for cloning and inspection:

```rust
pub struct OwnedMenu {
    pub name: SharedString,
    pub items: Vec<OwnedMenuItem>,
    pub disabled: bool,
}

pub enum OwnedMenuItem {
    Separator,
    Submenu(OwnedMenu),
    SystemMenu(OwnedOsMenu),
    Action {
        name: String,       // note: String, not SharedString
        action: Box<dyn Action>,
        os_action: Option<OsAction>,
        checked: bool,
        disabled: bool,
    },
}
```

Note that `OwnedMenuItem::Action.name` is `String`, while `MenuItem::Action.name` is `SharedString`.

### Keyboard shortcut ordering requirement

`cx.bind_keys()` **must** be called before `cx.set_menus()`. Internally, `set_menus()` passes the current keymap to the platform, which looks up bindings for each action to determine what shortcut to display. If bindings are added after `set_menus()`, the menu items will render without shortcut symbols.

### Shortcut display is single-keystroke only

Multi-keystroke sequences (chords like `"g g"`) cannot be displayed as menu shortcuts. macOS only supports single-keystroke key equivalents. If a binding has multiple keystrokes, the menu item renders with no shortcut shown.

### Test platform limitations

Both `set_menus()` and `get_menus()` are no-ops on the test platform. The keymap is also not publicly accessible via `App`. The pattern for testability is to extract both `menus()` and `key_bindings()` as pure functions returning data, and test them directly without going through the `App` context.
