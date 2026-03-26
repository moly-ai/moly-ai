use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ── Base ─────────────────────────────────────────────────────────────
    // Shared pill-shaped toggle with a sliding knob.
    // The `text` is set to a zero-width character so the Toggle renders
    // without a visible label by default (empty string doesn't work).

    SwitchBase = <Toggle> {
        text: "‎"

        draw_bg: {
            instance pill_color_off: (COLOR_DISABLED_BG)
            instance pill_color_on: (COLOR_PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                let pill_padding = 2.0;

                let pill_radius = self.rect_size.y * 0.5;
                let ball_radius = pill_radius - pill_padding;

                let pill_color = mix(
                    self.pill_color_off,
                    self.pill_color_on,
                    self.active
                );

                // Left cap
                sdf.circle(pill_radius, pill_radius, pill_radius);
                sdf.fill(pill_color);

                // Right cap
                sdf.circle(
                    self.rect_size.x - pill_radius,
                    pill_radius,
                    pill_radius
                );
                sdf.fill(pill_color);

                // Middle rect connecting the caps
                sdf.rect(
                    pill_radius,
                    0.0,
                    self.rect_size.x - 2.0 * pill_radius,
                    self.rect_size.y
                );
                sdf.fill(pill_color);

                // Sliding knob
                sdf.circle(
                    pill_padding
                        + ball_radius
                        + self.active
                            * (self.rect_size.x
                                - 2.0 * ball_radius
                                - 2.0 * pill_padding),
                    pill_radius,
                    ball_radius
                );
                sdf.fill(#fff);

                return sdf.result;
            }
        }

        animator: {
            active = {
                default: off
                off = {
                    ease: OutQuad
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { active: 0.0 } }
                }
                on = {
                    ease: OutQuad
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { active: 1.0 } }
                }
            }
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // VARIANTS
    // ═════════════════════════════════════════════════════════════════════

    pub Switch = <SwitchBase> {}

    pub DangerSwitch = <SwitchBase> {
        draw_bg: { pill_color_on: (COLOR_DANGER) }
    }

    pub WarningSwitch = <SwitchBase> {
        draw_bg: { pill_color_on: (COLOR_WARNING) }
    }

    pub NeutralSwitch = <SwitchBase> {
        draw_bg: { pill_color_on: (COLOR_NEUTRAL) }
    }
}
