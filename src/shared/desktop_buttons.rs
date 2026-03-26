use makepad_widgets::*;

// Overrides the dark theme colors of the desktop buttons.
// This is a temporary fix until the theme system allows for more flexible color overrides.
script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.MolyDesktopButton = DesktopButton {
        draw_bg +: {
            color: uniform(#5)
            color_hover: uniform(#d5)
            color_down: uniform(#c5)
        }
    }
}
