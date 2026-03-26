use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use crate::theme::*;

    pub Card = <RoundedShadowView> {
        width: Fit,
        height: Fit,
        padding: 16,
        flow: Down,

        draw_bg: {
            color: (COLOR_SURFACE)
            uniform border_radius: (RADIUS_CARD)
            uniform shadow_color: #0002
            uniform shadow_radius: 6.0
            uniform shadow_offset: vec2(0.0, 2.0)
        }
    }
}
