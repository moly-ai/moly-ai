use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.VerticalFiller = View {
        width: Fill
        height: 1
    }

    mod.widgets.HorizontalFiller = View {
        width: 1
        height: Fill
    }

    mod.widgets.Line = SolidView {
        width: Fill
        height: 1
        draw_bg +: {
            color: #D9D9D9
        }
    }

    mod.widgets.FadeView = CachedView {
        draw_bg +: {
            opacity: instance(1.0)

            pixel: fn() -> vec4 {
                let color = sample2d_rt(self.image self.pos * self.scale + self.shift)
                return Pal.premul(vec4(color.xyz color.w * self.opacity))
            }
        }
    }

    mod.widgets.AttributeTag = RoundedView {
        width: Fit
        height: Fit
        padding: Inset { top: 6 bottom: 6 left: 10 right: 10 }

        spacing: 5
        draw_bg +: {
            border_radius: instance(2.0)
        }

        attr_name := Label {
            draw_text +: {
                text_style: REGULAR_FONT { font_size: 8 }
                color: #x0
            }
        }
    }

    mod.widgets.SidebarMenuButton = RadioButton {
        width: 70
        height: Fit
        padding: 8
        margin: 0
        flow: Down
        spacing: 8.0
        align: Align { x: 0.5 y: 0.5 }

        icon_walk +: { margin: 0 width: 25 height: 25 }
        label_walk: Walk { margin: 0 }

        draw_bg +: {
            border_size: instance(0.0)
            border_color_1: instance(#0000)
            inset: instance(vec4(0.0 0.0 0.0 0.0))
            border_radius: instance(3.5)

            get_color: fn() -> vec4 {
                return mix(
                    mix(
                        #xf2f2f2
                        #x677483
                        self.hover
                    )
                    #x344054
                    self.active
                )
            }

            get_border_color: fn() -> vec4 {
                return self.border_color_1
            }

            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(
                    self.inset.x + self.border_size
                    self.inset.y + self.border_size
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_size * 2.0)
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_size * 2.0)
                    max(1.0 self.border_radius)
                )
                sdf.fill_keep(self.get_color())
                if self.border_size > 0.0 {
                    sdf.stroke(self.get_border_color() self.border_size)
                }
                return sdf.result
            }
        }

        draw_text +: {
            color: #x1A2533
            color_hover: uniform(#xF9F9F9)
            color_active: uniform(#xF9F9F9)
            hover: instance(0.0)
            active: instance(0.0)

            text_style: BOLD_FONT { font_size: 9 }

            get_color: fn() -> vec4 {
                return self.color
                    .mix(self.color_hover self.hover)
                    .mix(self.color_active self.active)
            }
        }

        draw_icon +: {
            color: #x1A2533
            color_hover: uniform(#xF9F9F9)
            color_active: uniform(#xF9F9F9)
            focus: instance(0.0)
            active: instance(0.0)
            get_color: fn() -> vec4 {
                return self.color
                    .mix(self.color_hover self.focus)
                    .mix(self.color_active self.active)
            }
        }
    }

    mod.widgets.MolyButton = Button {
        text: ""
        draw_bg +: {
            color: instance(#0000)
            color_hover: instance(#fff)
            border_size: instance(1.0)
            border_color_1: instance(#0000)
            border_color_hover: instance(#fff)
            border_radius: instance(2.5)

            get_color: fn() -> vec4 {
                return mix(
                    self.color
                    mix(self.color self.color_hover 0.2)
                    self.hover
                )
            }

            get_border_color: fn() -> vec4 {
                return mix(
                    self.border_color_1
                    mix(self.border_color_1 self.border_color_hover 0.2)
                    self.hover
                )
            }

            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(
                    self.border_size
                    self.border_size
                    self.rect_size.x - (self.border_size * 2.0)
                    self.rect_size.y - (self.border_size * 2.0)
                    max(1.0 self.border_radius)
                )
                sdf.fill_keep(self.get_color())
                if self.border_size > 0.0 {
                    sdf.stroke(self.get_border_color() self.border_size)
                }
                return sdf.result
            }
        }

        draw_icon +: {
            color: #fff
            color_hover: uniform(#000)
            hover: instance(0.0)
            rotation_angle: uniform(0.0)

            get_color: fn() -> vec4 {
                return mix(
                    self.color
                    mix(self.color self.color_hover 0.2)
                    self.hover
                )
            }

            clip_and_transform_vertex: fn(rect_pos: vec2 rect_size: vec2) -> vec4 {
                let clipped: vec2 = clamp(
                    self.geom_pos * rect_size + rect_pos
                    self.draw_clip.xy
                    self.draw_clip.zw
                )
                self.pos = (clipped - rect_pos) / rect_size

                let angle_rad = self.rotation_angle * 3.14159265359 / 180.0
                let cos_angle = cos(angle_rad)
                let sin_angle = sin(angle_rad)
                let rot_matrix = mat2(
                    cos_angle -sin_angle
                    sin_angle cos_angle
                )
                self.tex_coord1 = mix(
                    self.icon_t1.xy
                    self.icon_t2.xy
                    (rot_matrix * (self.pos.xy - vec2(0.5))) + vec2(0.5)
                )

                return self.camera_projection * (self.camera_view
                    * (self.view_transform * vec4(
                    clipped.x
                    clipped.y
                    self.draw_depth + self.draw_zbias
                    1.
                )))
            }
        }
        icon_walk +: { width: 14 height: 14 }

        draw_text +: {
            color: #fff
            text_style: REGULAR_FONT { font_size: 9 }
            get_color: fn() -> vec4 {
                return self.color
            }
        }

        reset_hover_on_click: true
    }

    mod.widgets.MolyRadioButtonTab = RadioButtonTab {
        padding: 10

        draw_bg +: {
            border_radius: uniform(3.0)
            border_size: uniform(0.0)
            color: instance(theme.color_text)
            color_hover: instance(theme.color_text_hover)

            get_color: fn() -> vec4 {
                return mix(
                    mix(
                        self.color
                        self.color_hover
                        self.hover
                    )
                    self.color_active
                    self.active
                )
            }

            get_border_color: fn() -> vec4 {
                return self.border_color
            }

            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(
                    self.border_size
                    self.border_size
                    self.rect_size.x - (self.border_size * 2.0)
                    self.rect_size.y - (self.border_size * 2.0)
                    max(1.0 self.border_radius)
                )
                sdf.fill_keep(self.get_color())
                if self.border_size > 0.0 {
                    sdf.stroke(self.get_border_color() self.border_size)
                }
                return sdf.result
            }
        }

        draw_text +: {
            get_color: fn() -> vec4 {
                return mix(
                    mix(
                        self.color
                        self.color_hover
                        self.hover
                    )
                    self.color_active
                    self.active
                )
            }
        }
    }

    mod.widgets.MolyTextInput = TextInput {
        padding: 5.0
        draw_text +: {
            text_style: REGULAR_FONT { font_size: 12 }

            color: #333
            color_hover: uniform(#222)
            color_focus: uniform(#222)
            color_down: uniform(#3)
            color_disabled: uniform(#8)
            color_empty: uniform(#8)
            color_empty_hover: uniform(#8)
            color_empty_focus: uniform(#8)
        }

        draw_cursor +: {
            focus: instance(0.0)
            border_radius: uniform(0.5)
            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(
                    0.
                    0.
                    self.rect_size.x
                    self.rect_size.y
                    self.border_radius
                )
                sdf.fill(mix(#fff #bbb self.focus))
                return sdf.result
            }
        }

        draw_selection +: {
            hover: instance(0.0)
            focus: instance(0.0)
            border_radius: uniform(2.0)
            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(
                    0.
                    0.
                    self.rect_size.x
                    self.rect_size.y
                    self.border_radius
                )
                sdf.fill(mix(#xee #xdd self.focus))
                return sdf.result
            }
        }

        draw_bg +: {
            color: #fff
            border_radius: instance(2.0)
            border_size: instance(0.0)
            border_color_1: instance(#3)
            inset: instance(vec4(0.0 0.0 0.0 0.0))

            get_color: fn() -> vec4 {
                return self.color
            }

            get_border_color: fn() -> vec4 {
                return self.border_color_1
            }

            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(
                    self.inset.x + self.border_size
                    self.inset.y + self.border_size
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_size * 2.0)
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_size * 2.0)
                    max(1.0 self.border_radius)
                )
                sdf.fill_keep(self.get_color())
                if self.border_size > 0.0 {
                    sdf.stroke(self.get_border_color() self.border_size)
                }
                return sdf.result
            }
        }
    }

    mod.widgets.MolySlider = Slider {
        height: 40
        width: Fill
        draw_text +: {
            text_style: BOLD_FONT { font_size: 10 }
            color: #000
        }
        text_input +: {
            draw_text +: {
                text_style: BOLD_FONT { font_size: 11 }
                get_color: fn() -> vec4 {
                    return #000
                }
            }
        }
        draw_bg +: {
            bipolar: instance(0.0)
            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let ball_radius = 10.0
                let ball_border = 2.0
                let padding_top = 29.0
                let padding_x = 5.0
                let rail_height = 4.0

                let rail_width = self.rect_size.x
                let rail_padding_x = padding_x + ball_radius / 2
                let ball_rel_x = self.slide_pos
                let ball_abs_x = ball_rel_x
                    * (rail_width - 2.0 * rail_padding_x) + rail_padding_x

                sdf.move_to(0 + padding_x padding_top)
                sdf.line_to(self.rect_size.x - padding_x padding_top)
                sdf.stroke(#D9D9D9 rail_height)

                sdf.move_to(0 + padding_x padding_top)
                sdf.line_to(ball_abs_x padding_top)
                sdf.stroke(#x15859A rail_height)

                sdf.circle(ball_abs_x padding_top ball_radius)
                sdf.fill(#x15859A)
                sdf.circle(ball_abs_x padding_top ball_radius - ball_border)
                sdf.fill(#fff)

                return sdf.result
            }
        }
    }

    mod.widgets.MolySwitch = Toggle {
        text: "\u{200e}"
        draw_bg +: {
            pixel: fn() -> vec4 {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let pill_padding = 2.0
                let pill_color_off = #D9D9D9
                let pill_color_on = #x429E92

                let pill_radius = self.rect_size.y * 0.5
                let ball_radius = pill_radius - pill_padding

                sdf.circle(pill_radius pill_radius pill_radius)
                sdf.fill(mix(pill_color_off pill_color_on self.active))

                sdf.circle(self.rect_size.x - pill_radius pill_radius pill_radius)
                sdf.fill(mix(pill_color_off pill_color_on self.active))

                sdf.rect(
                    pill_radius 0.0
                    self.rect_size.x - 2.0 * pill_radius
                    self.rect_size.y
                )
                sdf.fill(mix(pill_color_off pill_color_on self.active))

                sdf.circle(
                    pill_padding + ball_radius + self.active
                        * (self.rect_size.x - 2.0 * ball_radius - 2.0 * pill_padding)
                    pill_radius
                    ball_radius
                )
                sdf.fill(#fff)

                return sdf.result
            }
        }
    }

    mod.widgets.TogglePanelButton = mod.widgets.MolyButton {
        width: Fit
        height: Fit
        icon_walk +: { width: 20 height: 20 }
        draw_icon +: {
            get_color: fn() -> vec4 {
                return #475467
            }
        }
    }

    // TODO: TogglePanel widget was removed from new Makepad. MolyTogglePanel is
    // temporarily stubbed as a plain View. Consumers (chat_params.rs,
    // chat_history_panel.rs) will need to be updated to use a replacement widget.
    mod.widgets.MolyTogglePanel = View {}
}
