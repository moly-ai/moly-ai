use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use crate::theme::*;

    // ── Base ─────────────────────────────────────────────────────────────
    // Shared layout, padding, font, and radius for all Moly buttons.
    // Not public — use the specific variants below.

    MolyButtonBase = <Button> {
        width: Fit,
        height: Fit,
        padding: { top: 8, bottom: 8, left: 16, right: 16 }
        align: { x: 0.5, y: 0.5 }
        spacing: 6

        draw_bg: {
            uniform border_radius: (RADIUS_BUTTON)

            // Disable the 2-color border gradient inherited from Button.
            // The sentinel value (-1) makes the shader use border_color for
            // both top and bottom, producing a uniform single-color border.
            border_color_2: vec4(-1.0, -1.0, -1.0, -1.0)
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 10
            }
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // FILLED BUTTONS
    // Solid background, white text. Hover/down brighten the background.
    // ═════════════════════════════════════════════════════════════════════

    pub FilledButton = <MolyButtonBase> {
        draw_bg: {
            border_size: 0.0
            color: (COLOR_PRIMARY)
            color_hover: (COLOR_PRIMARY_HOVER)
            color_down: (COLOR_PRIMARY_DOWN)
            color_focus: (COLOR_PRIMARY)
            color_disabled: (COLOR_DISABLED_BG)
            border_color: (COLOR_TRANSPARENT)
            border_color_hover: (COLOR_TRANSPARENT)
            border_color_down: (COLOR_TRANSPARENT)
            border_color_focus: (COLOR_TRANSPARENT)
            border_color_disabled: (COLOR_TRANSPARENT)
        }
        draw_text: {
            color: (COLOR_TEXT_ON_FILLED)
            color_hover: (COLOR_TEXT_ON_FILLED)
            color_down: (COLOR_TEXT_ON_FILLED)
            color_focus: (COLOR_TEXT_ON_FILLED)
            color_disabled: (COLOR_DISABLED_TEXT)
        }
        draw_icon: {
            color: (COLOR_TEXT_ON_FILLED)
            color_hover: (COLOR_TEXT_ON_FILLED)
            color_down: (COLOR_TEXT_ON_FILLED)
            color_focus: (COLOR_TEXT_ON_FILLED)
            color_disabled: (COLOR_DISABLED_TEXT)
        }
    }

    pub FilledDangerButton = <FilledButton> {
        draw_bg: {
            color: (COLOR_DANGER)
            color_hover: (COLOR_DANGER_HOVER)
            color_down: (COLOR_DANGER_DOWN)
            color_focus: (COLOR_DANGER)
        }
    }

    pub FilledWarningButton = <FilledButton> {
        draw_bg: {
            color: (COLOR_WARNING)
            color_hover: (COLOR_WARNING_HOVER)
            color_down: (COLOR_WARNING_DOWN)
            color_focus: (COLOR_WARNING)
        }
    }

    pub FilledNeutralButton = <FilledButton> {
        draw_bg: {
            color: (COLOR_NEUTRAL)
            color_hover: (COLOR_NEUTRAL_HOVER)
            color_down: (COLOR_NEUTRAL_DOWN)
            color_focus: (COLOR_NEUTRAL)
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // OUTLINED BUTTONS
    // Transparent background with a colored border. Text matches the
    // border color. Hover/down fill with a muted tint of the color.
    // ═════════════════════════════════════════════════════════════════════

    pub OutlinedButton = <MolyButtonBase> {
        draw_bg: {
            border_size: 1.0
            color: (COLOR_TRANSPARENT)
            color_hover: (COLOR_PRIMARY_MUTED)
            color_down: (COLOR_PRIMARY_MUTED_HOVER)
            color_focus: (COLOR_TRANSPARENT)
            color_disabled: (COLOR_TRANSPARENT)
            border_color: (COLOR_PRIMARY)
            border_color_hover: (COLOR_PRIMARY_HOVER)
            border_color_down: (COLOR_PRIMARY_DOWN)
            border_color_focus: (COLOR_PRIMARY)
            border_color_disabled: (COLOR_DISABLED_BORDER)
        }
        draw_text: {
            color: (COLOR_PRIMARY)
            color_hover: (COLOR_PRIMARY_HOVER)
            color_down: (COLOR_PRIMARY_DOWN)
            color_focus: (COLOR_PRIMARY)
            color_disabled: (COLOR_DISABLED_TEXT)
        }
        draw_icon: {
            color: (COLOR_PRIMARY)
            color_hover: (COLOR_PRIMARY_HOVER)
            color_down: (COLOR_PRIMARY_DOWN)
            color_focus: (COLOR_PRIMARY)
            color_disabled: (COLOR_DISABLED_TEXT)
        }
    }

    pub OutlinedDangerButton = <OutlinedButton> {
        draw_bg: {
            color_hover: (COLOR_DANGER_MUTED)
            color_down: (COLOR_DANGER_MUTED_HOVER)
            border_color: (COLOR_DANGER)
            border_color_hover: (COLOR_DANGER_HOVER)
            border_color_down: (COLOR_DANGER_DOWN)
            border_color_focus: (COLOR_DANGER)
        }
        draw_text: {
            color: (COLOR_DANGER)
            color_hover: (COLOR_DANGER_HOVER)
            color_down: (COLOR_DANGER_DOWN)
            color_focus: (COLOR_DANGER)
        }
        draw_icon: {
            color: (COLOR_DANGER)
            color_hover: (COLOR_DANGER_HOVER)
            color_down: (COLOR_DANGER_DOWN)
            color_focus: (COLOR_DANGER)
        }
    }

    pub OutlinedWarningButton = <OutlinedButton> {
        draw_bg: {
            color_hover: (COLOR_WARNING_MUTED)
            color_down: (COLOR_WARNING_MUTED_HOVER)
            border_color: (COLOR_WARNING)
            border_color_hover: (COLOR_WARNING_HOVER)
            border_color_down: (COLOR_WARNING_DOWN)
            border_color_focus: (COLOR_WARNING)
        }
        draw_text: {
            color: (COLOR_WARNING)
            color_hover: (COLOR_WARNING_HOVER)
            color_down: (COLOR_WARNING_DOWN)
            color_focus: (COLOR_WARNING)
        }
        draw_icon: {
            color: (COLOR_WARNING)
            color_hover: (COLOR_WARNING_HOVER)
            color_down: (COLOR_WARNING_DOWN)
            color_focus: (COLOR_WARNING)
        }
    }

    pub OutlinedNeutralButton = <OutlinedButton> {
        draw_bg: {
            color_hover: (COLOR_NEUTRAL_MUTED)
            color_down: (COLOR_NEUTRAL_MUTED_HOVER)
            border_color: (COLOR_NEUTRAL)
            border_color_hover: (COLOR_NEUTRAL_HOVER)
            border_color_down: (COLOR_NEUTRAL_DOWN)
            border_color_focus: (COLOR_NEUTRAL)
        }
        draw_text: {
            color: (COLOR_NEUTRAL)
            color_hover: (COLOR_NEUTRAL_HOVER)
            color_down: (COLOR_NEUTRAL_DOWN)
            color_focus: (COLOR_NEUTRAL)
        }
        draw_icon: {
            color: (COLOR_NEUTRAL)
            color_hover: (COLOR_NEUTRAL_HOVER)
            color_down: (COLOR_NEUTRAL_DOWN)
            color_focus: (COLOR_NEUTRAL)
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // TEXT BUTTONS
    // No background, no border. Text is colored. Hover/down show a
    // subtle muted tint background.
    // ═════════════════════════════════════════════════════════════════════

    pub TextButton = <MolyButtonBase> {
        draw_bg: {
            border_size: 0.0
            color: (COLOR_TRANSPARENT)
            color_hover: (COLOR_PRIMARY_MUTED)
            color_down: (COLOR_PRIMARY_MUTED_HOVER)
            color_focus: (COLOR_TRANSPARENT)
            color_disabled: (COLOR_TRANSPARENT)
            border_color: (COLOR_TRANSPARENT)
            border_color_hover: (COLOR_TRANSPARENT)
            border_color_down: (COLOR_TRANSPARENT)
            border_color_focus: (COLOR_TRANSPARENT)
            border_color_disabled: (COLOR_TRANSPARENT)
        }
        draw_text: {
            color: (COLOR_PRIMARY)
            color_hover: (COLOR_PRIMARY_HOVER)
            color_down: (COLOR_PRIMARY_DOWN)
            color_focus: (COLOR_PRIMARY)
            color_disabled: (COLOR_DISABLED_TEXT)
        }
        draw_icon: {
            color: (COLOR_PRIMARY)
            color_hover: (COLOR_PRIMARY_HOVER)
            color_down: (COLOR_PRIMARY_DOWN)
            color_focus: (COLOR_PRIMARY)
            color_disabled: (COLOR_DISABLED_TEXT)
        }
    }

    pub TextDangerButton = <TextButton> {
        draw_bg: {
            color_hover: (COLOR_DANGER_MUTED)
            color_down: (COLOR_DANGER_MUTED_HOVER)
        }
        draw_text: {
            color: (COLOR_DANGER)
            color_hover: (COLOR_DANGER_HOVER)
            color_down: (COLOR_DANGER_DOWN)
            color_focus: (COLOR_DANGER)
        }
        draw_icon: {
            color: (COLOR_DANGER)
            color_hover: (COLOR_DANGER_HOVER)
            color_down: (COLOR_DANGER_DOWN)
            color_focus: (COLOR_DANGER)
        }
    }

    pub TextWarningButton = <TextButton> {
        draw_bg: {
            color_hover: (COLOR_WARNING_MUTED)
            color_down: (COLOR_WARNING_MUTED_HOVER)
        }
        draw_text: {
            color: (COLOR_WARNING)
            color_hover: (COLOR_WARNING_HOVER)
            color_down: (COLOR_WARNING_DOWN)
            color_focus: (COLOR_WARNING)
        }
        draw_icon: {
            color: (COLOR_WARNING)
            color_hover: (COLOR_WARNING_HOVER)
            color_down: (COLOR_WARNING_DOWN)
            color_focus: (COLOR_WARNING)
        }
    }

    pub TextNeutralButton = <TextButton> {
        draw_bg: {
            color_hover: (COLOR_NEUTRAL_MUTED)
            color_down: (COLOR_NEUTRAL_MUTED_HOVER)
        }
        draw_text: {
            color: (COLOR_NEUTRAL)
            color_hover: (COLOR_NEUTRAL_HOVER)
            color_down: (COLOR_NEUTRAL_DOWN)
            color_focus: (COLOR_NEUTRAL)
        }
        draw_icon: {
            color: (COLOR_NEUTRAL)
            color_hover: (COLOR_NEUTRAL_HOVER)
            color_down: (COLOR_NEUTRAL_DOWN)
            color_focus: (COLOR_NEUTRAL)
        }
    }
}
