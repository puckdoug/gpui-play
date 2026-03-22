# MenuTest - Test Infrastructure & Menu Tests (Red Phase)

## Context

Setting up test infrastructure for the gpui-play project and writing the first TDD red-phase tests for a `MenuTest` program. The MenuTest binary demonstrates GPUI menu creation. We write failing tests first, then implement.

## What We're Building

Binary: `src/bin/menu_test.rs` (run via `cargo run --bin menu_test`)

Tests verify:
1. Program creates 4 top-level menus: MenuTest, File, Edit, Help
2. MenuTest menu has "About MenuTest" item
3. File menu has "Quit" item
4. Edit menu has: Undo, Redo, Cut, Copy, Paste
5. Help menu has a "Search" item
6. Only Quit has an action handler — all other items are disabled
7. Quit action quits the program

## Files to Create/Modify

### 1. `Cargo.toml` — Add test-support feature and dependencies
```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
gpui_platform = { git = "https://github.com/zed-industries/zed" }

[dev-dependencies]
gpui = { git = "https://github.com/zed-industries/zed", features = ["test-support"] }
```

### 2. `src/lib.rs` — Library root
```rust
pub mod menu_test;
```
Exports the menu_test module so tests in `src/bin/menu_test.rs` and the lib's test harness can access it.

### 3. `src/menu_test.rs` — Module with menu setup logic + tests

Contains:
- `actions!()` macro defining: `Quit`, `About`, `Undo`, `Redo`, `Cut`, `Copy`, `Paste`, `Search`
- `setup_menus(cx: &mut App)` — sets menus via `cx.set_menus(...)` and registers the Quit action handler
- `#[cfg(test)] mod tests` with the red-phase tests

**Actions:**
```rust
actions!(menu_test, [Quit, About, Undo, Redo, Cut, Copy, Paste, Search]);
```

**Menu structure built by `setup_menus()`:**
```rust
cx.set_menus([
    Menu::new("MenuTest").items([
        MenuItem::action("About MenuTest", About).disabled(true),
    ]),
    Menu::new("File").items([
        MenuItem::action("Quit", Quit),  // only enabled item
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
]);
cx.on_action(|_: &Quit, cx: &mut App| cx.quit());
```

### 4. `src/bin/menu_test.rs` — Binary entry point

Minimal binary that calls the setup function and opens a window:
```rust
use gpui_play::menu_test;
// ... bootstrap and call menu_test::setup_menus(cx)
```

## Test Specifications (`src/menu_test.rs` — `#[cfg(test)] mod tests`)

Uses `#[gpui::test]` macro and `TestAppContext`.

**Helper function:** `find_menu(menus, name) -> &OwnedMenu` — finds a menu by name in the vec.
**Helper function:** `find_action_item(menu, name) -> (name, disabled)` — finds an action item by name and returns its disabled state.

### Test 1: `test_creates_four_menus`
```rust
cx.update(|cx| setup_menus(cx));
cx.read(|cx| {
    let menus = cx.get_menus().expect("menus should be set");
    assert_eq!(menus.len(), 4);
    assert_eq!(menus[0].name.as_ref(), "MenuTest");
    assert_eq!(menus[1].name.as_ref(), "File");
    assert_eq!(menus[2].name.as_ref(), "Edit");
    assert_eq!(menus[3].name.as_ref(), "Help");
});
```

### Test 2: `test_menutest_menu_has_about`
Verifies the MenuTest menu contains "About MenuTest" action item.

### Test 3: `test_file_menu_has_quit`
Verifies the File menu contains "Quit" action item.

### Test 4: `test_edit_menu_has_expected_items`
Verifies Edit menu contains Undo, Redo, Cut, Copy, Paste (filtering out separators).

### Test 5: `test_help_menu_has_search`
Verifies Help menu contains "Search" action item.

### Test 6: `test_only_quit_is_enabled`
Iterates all menus and all action items. Asserts that only "Quit" has `disabled == false`, all others have `disabled == true`.

### Test 7: `test_quit_action_quits`
Registers the quit handler, dispatches the Quit action, verifies `cx.quit()` was called. (This may need a window + VisualTestContext to dispatch the action properly.)

## Red Phase Strategy

For the red phase, we:
1. Create `Cargo.toml` with deps (so it compiles)
2. Create `src/lib.rs` with `pub mod menu_test;`
3. Create `src/menu_test.rs` with:
   - The `actions!()` macro (needed for tests to compile)
   - An empty `pub fn setup_menus(cx: &mut App) {}` stub
   - All 7 tests in `#[cfg(test)] mod tests`
4. Run `cargo test` — tests compile but fail (red)

The stub `setup_menus` does nothing, so `cx.get_menus()` returns `None` and all asserts fail.

## Verification
- `cargo test` — all 7 tests should compile and fail (red phase)
- `cargo check --bin menu_test` — binary should compile (even if minimal)
