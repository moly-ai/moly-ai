use makepad_widgets::{
    image_cache::{ImageBuffer, ImageError},
    *,
};

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.ImageView = #(ImageView::register_widget(vm)) View {
        align: Align { x: 0.5, y: 0.5 }
        image := Image { width: 0, height: 0 }
    }
}

/// A wrapped image widget, where its inner [`Image`] is calculated to an exact size.
///
/// Therefore is affected by certain properties in its wrapper [`View`] such as `align`
/// or `padding` instead of being always `Fill` with changes in the shader.
#[derive(Script, ScriptHook, Widget)]
pub struct ImageView {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    deref: View,

    // TODO: Make an enum with `Contain` and `Cover` variants.
    #[live]
    pub contain: bool,

    #[rust]
    texture: Option<Texture>,
}

impl Widget for ImageView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.peek_walk_turtle(walk);
        let available_width = rect.size.x;
        let available_height = rect.size.y;

        let dpi = cx.current_dpi_factor();
        let (image_width, image_height) = self.image_size(cx);
        let image_width = image_width as f64 * dpi;
        let image_height = image_height as f64 * dpi;

        let scale_x = available_width / image_width;
        let scale_y = available_height / image_height;

        let scale = if self.contain {
            scale_x.min(scale_y).clamp(0.0, 1.0)
        } else {
            scale_x.max(scale_y)
        };

        let scaled_width = image_width * scale;
        let scaled_height = image_height * scale;

        let mut image = self.image_ref(cx);
        script_apply_eval!(cx, image, {
            width: #(scaled_width)
            height: #(scaled_height)
        });

        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope)
    }
}

impl ImageView {
    pub fn load_png(&mut self, cx: &mut Cx, data: &[u8]) -> Result<(), ImageError> {
        self.load_buffer(cx, ImageBuffer::from_png(data)?);
        Ok(())
    }

    pub fn load_jpeg(&mut self, cx: &mut Cx, data: &[u8]) -> Result<(), ImageError> {
        self.load_buffer(cx, ImageBuffer::from_jpg(data)?);
        Ok(())
    }

    pub fn load_with_contet_type(
        &mut self,
        cx: &mut Cx,
        data: &[u8],
        content_type: &str,
    ) -> Result<(), ImageError> {
        if can_load(content_type) {
            match content_type {
                "image/png" => self.load_png(cx, data),
                "image/jpeg" => self.load_jpeg(cx, data),
                _ => Err(ImageError::UnsupportedFormat),
            }
        } else {
            Err(ImageError::UnsupportedFormat)
        }
    }

    fn load_buffer(&mut self, cx: &mut Cx, buffer: ImageBuffer) {
        let texture = buffer.into_new_texture(cx);
        self.set_texture(cx, Some(texture));
    }

    pub fn set_texture(&mut self, cx: &mut Cx, texture: Option<Texture>) {
        self.texture = texture;
        self.image_ref(cx).set_texture(cx, self.texture.clone());
    }

    #[allow(dead_code)]
    pub fn get_texture(&self) -> Option<Texture> {
        self.texture.clone()
    }

    fn image_ref(&self, cx: &Cx) -> ImageRef {
        self.image(cx, ids!(image))
    }

    fn image_size(&self, cx: &mut Cx) -> (usize, usize) {
        self.texture
            .as_ref()
            .and_then(|t| t.get_format(cx).vec_width_height())
            .unwrap_or((0, 0))
    }
}

/// If this image widget supports the given content type.
pub fn can_load(content_type: &str) -> bool {
    matches!(content_type, "image/png" | "image/jpeg")
}
