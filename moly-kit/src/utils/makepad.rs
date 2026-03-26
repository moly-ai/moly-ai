//! Utilities to deal with stuff that is highly specific to Makepad.

pub mod events;
pub mod hits;
pub mod portal_list;
pub mod ui_runner;

use makepad_widgets::*;
use std::path::Path;
use std::sync::Arc;

/// Convert from hex color notation to makepad's Vec4 color.
/// Ex: Converts `0xff33cc` into `vec4(1.0, 0.2, 0.8, 1.0)`.
pub fn hex_rgb_color(hex: u32) -> Vec4 {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    vec4(r, g, b, 1.0)
}

/// Loads an image into an [`ImageRef`] from a resource registered via
/// `crate_resource()`.
///
/// `abs_path` is the absolute filesystem path stored as the resource's
/// `abs_path` when it was registered with the script resource system.
/// The function looks up the corresponding [`ScriptHandle`], ensures
/// the resource data is loaded, and feeds the bytes to the image
/// widget's async decoder.
///
/// # Errors
///
/// Returns [`ImageError::PathNotFound`] if no script resource matches
/// `abs_path`, or [`ImageError::NotYetLoaded`] if the resource exists
/// but its data is not available yet (e.g. pending HTTP fetch on web).
pub fn load_image_from_resource(
    image: &ImageRef,
    cx: &mut Cx,
    abs_path: &str,
) -> Result<(), ImageError> {
    let handle = cx
        .script_data
        .resources
        .get_handle_by_abs_path(abs_path)
        .ok_or_else(|| ImageError::PathNotFound(abs_path.into()))?;
    cx.load_script_resource(handle);
    let data = cx
        .get_resource(handle)
        .ok_or(ImageError::NotYetLoaded)?;
    let path = Path::new(abs_path);
    let data = Arc::new((*data).clone());
    image.load_image_from_data_async(cx, path, data)
}
