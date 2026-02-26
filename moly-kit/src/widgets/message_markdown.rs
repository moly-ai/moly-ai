use makepad_widgets::*;
script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    let MD_LINE_SPACING = 1.5
    let MD_FONT_COLOR = #000

    mod.widgets.MessageMarkdown = mod.widgets.Markdown {
        padding: 0
        margin: 0
        paragraph_spacing: 16
        heading_base_scale: 1.6

        font_color: #000
        width: Fill height: Fit
        font_size: 11.0
        code_block := View {
            width: Fill
            height: Fit
            flow: Down
            RoundedView {
                draw_bg +: {
                    radius: 0.0
                    border_size: 1.2
                    border_color: #x1d2330
                }
                width: Fill
                height: Fit
                align: Align{x: 1.0}

                copy_code_button := ButtonFlat {
                    margin: Inset{right: 2}
                    draw_bg +: {
                        border_size: 0.0
                    }
                    icon_walk: Walk{
                        width: 12 height: Fit
                        margin: Inset{left: 10}
                    }
                    draw_icon +: {
                        color: #x0
                        color_hover: #x3c3c3c
                        color_down: #x0
                        color_focus: #x0
                        svg_file: crate_resource("self://resources/copy.svg")
                    }
                }
            }
            code_view := CodeView {
                editor +: {
                    margin: Inset{top: -2 bottom: 2}
                    pad_left_top: vec2(10.0 10.0)
                    width: Fill
                    height: Fit
                    draw_bg +: {color: #x1d2330}
                    draw_text +: {
                        text_style +: {
                            font_size: 10
                        }
                    }

                    token_colors +: {
                        whitespace: #xa8b5d1
                        delimiter: #xa8b5d1
                        delimiter_highlight: #xc5cee0
                        error_decoration: #xf44747
                        warning_decoration: #xcd9731

                        unknown: #xa8b5d1
                        branch_keyword: #xd2a6ef
                        constant: #xffd9af
                        identifier: #xa8b5d1
                        loop_keyword: #xd2a6ef
                        number: #xffd9af
                        other_keyword: #xd2a6ef
                        punctuator: #xa8b5d1
                        string: #x58ffc7
                        function: #x82aaff
                        typename: #xfcf9c3
                        comment: #x506686
                    }
                }
            }
        }
        use_code_block_widget: true

        use_math_widget: true
        inline_math := MathView {
            color: #000
            font_size: 11.0
        }
        display_math := MathView {
            color: #000
            font_size: 11.0
        }

        list_item_layout: Layout{
            padding: Inset{left: 10.0 right: 10 top: 6.0 bottom: 0}
        }
        list_item_walk: Walk{margin: 0 height: Fit width: Fill}
        code_layout: Layout{
            padding: Inset{top: 10.0 bottom: 10.0}
        }
        quote_layout: Layout{
            padding: Inset{top: 10.0 bottom: 10.0}
        }

        link := mod.widgets.MarkdownLink {
            padding: Inset{top: 1 bottom: 0}
            draw_text +: {
                color: #x00f
                color_pressed: #xf00
                color_hover: #x0f0
            }
        }

        text_style_normal +: {
            line_spacing: 1.5
        }
        text_style_italic +: {
            line_spacing: 1.5
        }
        text_style_bold +: {
            line_spacing: 1.5
        }
        text_style_bold_italic +: {
            line_spacing: 1.5
        }
        text_style_fixed +: {
            line_spacing: 1.5
        }
        draw_block +: {
            line_color: #000
            sep_color: #xEDEDED
            quote_bg_color: #xEDEDED
            quote_fg_color: #x969696
            code_color: #xEDEDED
        }
    }
}
