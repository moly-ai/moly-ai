use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.Label = set_type_default() do mod.widgets.Label {
        padding: 0
    }
}
