use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ModelLink = View {
        width: Fit
        height: Fit
        flow: Down
        link := LinkLabel {
            width: Fit
            margin: 2
            draw_text +: {
                text_style: theme.font_regular { font_size: 9 }
                get_color: fn() -> vec4 {
                    return mix(
                        mix(
                            MODEL_LINK_FONT_COLOR
                            MODEL_LINK_FONT_COLOR
                            self.hover
                        )
                        MODEL_LINK_FONT_COLOR
                        self.down
                    )
                }
            }
        }
        underline := Line {
            width: Fill
            height: 1
            show_bg: true
            draw_bg +: {
                color: (MODEL_LINK_FONT_COLOR)
            }
        }
    }

    mod.widgets.ModelAttributeTag = RoundedView {
        width: Fit
        height: Fit
        padding: Inset { top: 6 bottom: 6 left: 10 right: 10 }

        spacing: 5
        draw_bg +: {
            border_radius: instance(3.0)
        }

        attr_name := Label {
            draw_text +: {
                text_style: theme.font_regular { font_size: 9 }
                color: #x0
            }
        }

        attr_value := Label {
            draw_text +: {
                text_style: theme.font_bold { font_size: 9 }
                color: #x0
            }
        }
    }

    mod.widgets.ModelAttributes = View {
        width: Fit
        height: Fit
        spacing: 8

        model_size_tag := mod.widgets.ModelAttributeTag {
            draw_bg +: { color: #D4E6F7 }
            attr_name +: { text: "Model Size" }
        }

        model_requires_tag := mod.widgets.ModelAttributeTag {
            draw_bg +: { color: #D6F5EB }
            attr_name +: { text: "Requires" }
        }

        model_architecture_tag := mod.widgets.ModelAttributeTag {
            draw_bg +: { color: #F0D6F5 }
            attr_name +: { text: "Architecture" }
        }
    }
}
