# `load_image_dep_by_path` is non-functional in the new Makepad system

## Status

**Open** — reported during Moly migration to `script_mod!`.

## Summary

`ImageRef::load_image_dep_by_path()` always fails with `"Dependency not loaded"` in the
new `script_mod!` / Splash system because `cx.dependencies` (the HashMap it reads from)
is never populated.

## Root Cause

In the old `live_design!` system, `dep("crate://self/path")` declarations were collected by
`live_scan_dependencies()` into `cx.dependencies`, and then platform init code loaded the
actual file bytes into each entry. `load_image_dep_by_path` retrieved those bytes via
`cx.take_dependency(path)`.

In the new system:

- `live_scan_dependencies()` **does not exist** in the new `platform/src/` code. On macOS
  and Windows the call is **commented out** in the platform init files.
- `crate_resource("self://path")` writes to `cx.script_data.resources` — a completely
  separate system. It **never populates** `cx.dependencies`.
- `cx.dependencies` is initialized empty and stays empty on all major platforms.
- Makepad's own `Image` widget no longer uses `load_image_dep_by_path` internally — it
  uses `load_from_resource()` with `ScriptHandleRef` from the `src` field.

## Evidence

| Location | Detail |
|----------|--------|
| `platform/src/cx.rs:103` | `dependencies: HashMap<String, CxDependency>` — initialized empty (line 395) |
| `platform/src/cx_api.rs:328-359` | `take_dependency()` reads from `self.dependencies`, returns Err if empty |
| `platform/src/os/apple/macos/macos.rs:1516` | `//self.live_scan_dependencies();` — commented out |
| `platform/src/os/windows/windows.rs:808` | `//self.live_scan_dependencies();` — commented out |
| `platform/src/script/res.rs:751` | `crate_resource()` writes to `cx.script_data.resources`, not `cx.dependencies` |
| `draw/src/image_cache.rs:807-822` | `load_image_dep_by_path` impl on `ImageCacheImpl` trait |
| `widgets/src/image.rs:378-381` | Image's `draw_walk` calls `load_from_resource()`, not `load_image_dep_by_path` |

## Impact

Any code calling `load_image_dep_by_path` will silently fail. In Moly, this affects
provider icon loading in:
- `moly-kit/src/widgets/avatar.rs`
- `moly-kit/src/widgets/model_selector_list.rs`
- `src/settings/providers.rs`

## Suggested Fix (for Makepad)

One of:

1. **Deprecate** `load_image_dep_by_path` and document that callers should use
   `src: crate_resource("self://path")` in DSL, or `load_png_from_data` /
   `load_image_file_by_path` from Rust.

2. **Bridge** the function to the new resource system — make `take_dependency` fall back
   to looking up `cx.script_data.resources` by path and returning the loaded data.

3. **Re-enable** dependency scanning by implementing a new version that reads from the
   script resource registry instead of the old live registry.

## Workaround (for Moly)

Use the `Image` widget's `src: crate_resource("self://path")` mechanism, declaring
provider icons as DSL resources and passing `ScriptHandleRef` handles to the avatar widget.
Alternatively, use `load_png_from_data` with bytes read from disk or embedded via
`include_bytes!`.
