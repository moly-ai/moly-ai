use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    // The default theme for MolyKit.
    //
    // Instead of overriding Makepad's default theme, we apply MolyKit-specific
    // widget overrides onto mod.widgets.
    //
    // This way, we can keep Makepad's default theme as is, to not interfere
    // with the rest of the application.
    //
    // TODO: In this file we'll set a set of rules/constants that will be used
    // to style MolyKit's widgets. We can also override Makepad's default theme
    // as needed (e.g. space_factor).
    //
    // Currently we're using this to globally (MolyKit-wide) override some
    // painful defaults in Makepad. Ideally we'd override some spacing values
    // in Makepad, but that doesn't seem to be enough, therefore we're also
    // overriding some widget-specific values here.
    mod.widgets.Label = mod.widgets.Label { padding: 0 }
}
