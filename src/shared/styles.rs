use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.TRANSPARENT = #0000
    mod.widgets.PRIMARY_COLOR = #3A7CA5

    mod.widgets.MODEL_LINK_FONT_COLOR = #x155EEF
    mod.widgets.SIDEBAR_BG_COLOR = #xf2f2f2
    mod.widgets.MAIN_BG_COLOR = #xf9f9f9
    mod.widgets.MAIN_BG_COLOR_DARK = #xf2f2f2
    mod.widgets.CTA_BUTTON_COLOR = mod.widgets.PRIMARY_COLOR

    mod.widgets.REGULAR_FONT = theme.font_regular{
        font_size: 12
    }

    mod.widgets.BOLD_FONT = theme.font_bold{
        font_size: 12
    }

    // Override the default label with no default padding
    // Ideally we'd replace THEME_MSPACE_1 or THEME_MSPACE_FACTOR, but it causes issues
    // elsewhere.
    // TODO: Introduce a proper theme in Moly that overrides default values to match our
    // general styling, instead of patching individual widgets everywhere.
    mod.widgets.Label = Label {
        padding: 0
    }

    mod.widgets.RoundedInnerShadowView = View {
        show_bg: true
        draw_bg +: {
            color: uniform(#8)
            border_radius: uniform(2.5)
            border_size: uniform(0.0)
            border_color: instance(#0000)
            shadow_color: uniform(#0007)
            shadow_radius: uniform(10.0)
            shadow_offset: uniform(vec2(0.0 0.0))

            get_color: fn() {
                return self.color
            }

            get_border_color: fn() {
                return self.border_color
            }

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let outer_x = self.border_size
                let outer_y = self.border_size
                let outer_w = self.rect_size.x - 2.0 * self.border_size
                let outer_h = self.rect_size.y - 2.0 * self.border_size
                let outer_rad = max(1.0 self.border_radius)

                sdf.box(outer_x outer_y outer_w outer_h outer_rad)
                let outer_dist = sdf.shape

                let base_color_raw = self.get_color()

                let shadow_blur = self.shadow_radius

                let dist_from_edge_inside = -outer_dist

                let intensity = 1.0 - smoothstep(
                    0.0
                    shadow_blur
                    dist_from_edge_inside
                )

                let shadow_factor = clamp(intensity 0.0 1.0)
                    * step(outer_dist 0.0)

                let effective_shadow_alpha = shadow_factor
                    * self.shadow_color.a

                let final_rgb = mix(
                    base_color_raw.rgb
                    self.shadow_color.rgb
                    effective_shadow_alpha
                )

                let final_color_raw = vec4(
                    final_rgb
                    base_color_raw.a
                )

                sdf.fill(final_color_raw)

                if self.border_size > 0.0 {
                    sdf.box(outer_x outer_y outer_w outer_h outer_rad)
                    sdf.stroke_keep(self.get_border_color() self.border_size)
                }

                return sdf.result
            }
        }
    }
}
