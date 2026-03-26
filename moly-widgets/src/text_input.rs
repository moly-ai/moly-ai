use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ── Base ─────────────────────────────────────────────────────────────
    // Material 2-inspired text input: white surface, subtle border, rounded
    // corners, and a colored border on focus.
    //
    // We write a custom draw_bg shader because the base TextInputFlat has
    // hover overriding focus (so focus+hover shows the plain border). Our
    // priority is: disabled > focus > hover > empty > base.

    TextInputBase = <TextInputFlat> {
        width: Fill,
        height: Fit,
        padding: { top: 10, bottom: 10, left: 12, right: 12 }
        margin: 0,

        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
            instance disabled: 0.0
            instance empty: 0.0

            uniform border_radius: (RADIUS_INPUT)
            uniform border_size: 1.0

            // ── Fill colors ──
            instance color: (COLOR_SURFACE)
            instance color_hover: (COLOR_SURFACE)
            instance color_focus: (COLOR_SURFACE)
            instance color_down: (COLOR_SURFACE)
            instance color_empty: (COLOR_SURFACE)
            instance color_disabled: (COLOR_DISABLED_BG)

            // ── Border colors ──
            instance border_color: (COLOR_BORDER)
            instance border_color_hover: (COLOR_BORDER)
            instance border_color_focus: (COLOR_PRIMARY)
            instance border_color_down: (COLOR_PRIMARY)
            instance border_color_empty: (COLOR_BORDER)
            instance border_color_disabled: (COLOR_DISABLED_BORDER)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Priority: disabled > focus > hover/down > empty > base
                let fill = self.color;
                let border = self.border_color;

                // Empty
                fill = mix(fill, self.color_empty, self.empty);
                border = mix(border, self.border_color_empty, self.empty);

                // Hover / down
                let hover_fill = mix(self.color_hover, self.color_down, self.down);
                let hover_border = mix(
                    self.border_color_hover,
                    self.border_color_down,
                    self.down
                );
                fill = mix(fill, hover_fill, self.hover);
                border = mix(border, hover_border, self.hover);

                // Focus (overrides hover)
                fill = mix(fill, self.color_focus, self.focus);
                border = mix(border, self.border_color_focus, self.focus);

                // Disabled (overrides everything)
                fill = mix(fill, self.color_disabled, self.disabled);
                border = mix(border, self.border_color_disabled, self.disabled);

                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.0,
                    self.rect_size.y - self.border_size * 2.0,
                    self.border_radius
                );

                sdf.fill_keep(fill);
                sdf.stroke(border, self.border_size);

                return sdf.result;
            }
        }

        draw_text: {
            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
            instance empty: 0.0
            instance disabled: 0.0

            text_style: <THEME_FONT_REGULAR> {
                font_size: 10
            }

            color: (COLOR_TEXT)
            uniform color_hover: (COLOR_TEXT)
            uniform color_focus: (COLOR_TEXT)
            uniform color_down: (COLOR_TEXT)
            uniform color_disabled: (COLOR_DISABLED_TEXT)
            uniform color_empty: (COLOR_DISABLED_TEXT)
            uniform color_empty_hover: (COLOR_DISABLED_TEXT)
            uniform color_empty_focus: (COLOR_TEXT)

            // Priority: disabled > focus > empty > hover > base
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(
                                self.color,
                                mix(
                                    self.color_hover,
                                    self.color_down,
                                    self.down
                                ),
                                self.hover
                            ),
                            self.color_empty,
                            self.empty
                        ),
                        self.color_focus,
                        self.focus
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }

        draw_cursor: {
            uniform color: (COLOR_PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                );
                sdf.fill(
                    mix(
                        #0000,
                        self.color,
                        (1.0 - self.blink) * self.focus * (1.0 - self.disabled)
                    )
                );
                return sdf.result;
            }
        }

        draw_selection: {
            uniform color: (COLOR_PRIMARY_MUTED)
            uniform color_focus: (COLOR_PRIMARY_MUTED_HOVER)
        }

        animator: {
            empty = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: {
                        draw_bg: { empty: 0.0 }
                        draw_text: { empty: 0.0 }
                        draw_selection: { empty: 0.0 }
                        draw_cursor: { empty: 0.0 }
                    }
                }
                on = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: {
                        draw_bg: { empty: 1.0 }
                        draw_text: { empty: 1.0 }
                        draw_selection: { empty: 1.0 }
                        draw_cursor: { empty: 1.0 }
                    }
                }
            }
            blink = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.05 } }
                    apply: { draw_cursor: { blink: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.05 } }
                    apply: { draw_cursor: { blink: 1.0 } }
                }
            }
            hover = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: {
                        draw_bg: { down: 0.0, hover: 0.0 }
                        draw_text: { down: 0.0, hover: 0.0 }
                    }
                }
                on = {
                    from: {
                        all: Forward { duration: 0.1 }
                        down: Forward { duration: 0.01 }
                    }
                    apply: {
                        draw_bg: { down: 0.0, hover: [{time: 0.0, value: 1.0}] }
                        draw_text: { down: 0.0, hover: [{time: 0.0, value: 1.0}] }
                    }
                }
                down = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: {
                        draw_bg: { down: [{time: 0.0, value: 1.0}], hover: 1.0 }
                        draw_text: { down: [{time: 0.0, value: 1.0}], hover: 1.0 }
                    }
                }
            }
            disabled = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.0 } }
                    apply: {
                        draw_bg: { disabled: 0.0 }
                        draw_text: { disabled: 0.0 }
                        draw_selection: { disabled: 0.0 }
                        draw_cursor: { disabled: 0.0 }
                    }
                }
                on = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: {
                        draw_bg: { disabled: 1.0 }
                        draw_text: { disabled: 1.0 }
                        draw_selection: { disabled: 1.0 }
                        draw_cursor: { disabled: 1.0 }
                    }
                }
            }
            focus = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: {
                        draw_bg: { focus: 0.0 }
                        draw_text: { focus: 0.0 }
                        draw_cursor: { focus: 0.0 }
                        draw_selection: { focus: 0.0 }
                    }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: {
                        draw_bg: { focus: 1.0 }
                        draw_text: { focus: 1.0 }
                        draw_cursor: { focus: 1.0 }
                        draw_selection: { focus: 1.0 }
                    }
                }
            }
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // VARIANTS
    // ═════════════════════════════════════════════════════════════════════

    pub PrimaryTextInput = <TextInputBase> {}

    pub NeutralTextInput = <TextInputBase> {
        draw_bg: {
            instance border_color_focus: (COLOR_NEUTRAL)
            instance border_color_down: (COLOR_NEUTRAL)
        }
        draw_cursor: {
            uniform color: (COLOR_NEUTRAL)
        }
        draw_selection: {
            uniform color: (COLOR_NEUTRAL_MUTED)
            uniform color_focus: (COLOR_NEUTRAL_MUTED_HOVER)
        }
    }

    pub DangerTextInput = <TextInputBase> {
        draw_bg: {
            instance border_color_focus: (COLOR_DANGER)
            instance border_color_down: (COLOR_DANGER)
        }
        draw_cursor: {
            uniform color: (COLOR_DANGER)
        }
        draw_selection: {
            uniform color: (COLOR_DANGER_MUTED)
            uniform color_focus: (COLOR_DANGER_MUTED_HOVER)
        }
    }

    pub WarningTextInput = <TextInputBase> {
        draw_bg: {
            instance border_color_focus: (COLOR_WARNING)
            instance border_color_down: (COLOR_WARNING)
        }
        draw_cursor: {
            uniform color: (COLOR_WARNING)
        }
        draw_selection: {
            uniform color: (COLOR_WARNING_MUTED)
            uniform color_focus: (COLOR_WARNING_MUTED_HOVER)
        }
    }

    pub TransparentTextInput = <TextInputBase> {
        draw_bg: {
            instance color: (COLOR_TRANSPARENT)
            instance color_hover: (COLOR_TRANSPARENT)
            instance color_focus: (COLOR_TRANSPARENT)
            instance color_down: (COLOR_TRANSPARENT)
            instance color_empty: (COLOR_TRANSPARENT)
            instance color_disabled: (COLOR_TRANSPARENT)

            instance border_color: (COLOR_TRANSPARENT)
            instance border_color_hover: (COLOR_TRANSPARENT)
            instance border_color_focus: (COLOR_TRANSPARENT)
            instance border_color_down: (COLOR_TRANSPARENT)
            instance border_color_empty: (COLOR_TRANSPARENT)
            instance border_color_disabled: (COLOR_TRANSPARENT)
        }
        draw_cursor: {
            uniform color: (COLOR_PRIMARY)
        }
        draw_selection: {
            uniform color: (COLOR_PRIMARY_MUTED)
            uniform color_focus: (COLOR_PRIMARY_MUTED_HOVER)
        }
    }
}
