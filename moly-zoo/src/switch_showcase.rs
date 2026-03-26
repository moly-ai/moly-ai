use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use moly_widgets::theme::*;
    use moly_widgets::switch::*;

    SectionLabel = <Label> {
        width: Fill,
        padding: { bottom: 4 }
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 11 }
            color: (COLOR_TEXT)
        }
    }

    SwitchRow = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 20,
        align: { y: 0.5 }
    }

    pub SwitchShowcase = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 24,
        padding: 32,

        <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 14 }
                color: (COLOR_TEXT)
            }
            text: "Switches"
        }

        // ── Color Variants ──────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Color Variants" }
            <SwitchRow> {
                <Switch> {}
                <DangerSwitch> {}
                <WarningSwitch> {}
                <NeutralSwitch> {}
            }
        }

        // ── Pre-activated ───────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Pre-activated (active: true)" }
            <SwitchRow> {
                <Switch> { active: true }
                <DangerSwitch> { active: true }
                <WarningSwitch> { active: true }
                <NeutralSwitch> { active: true }
            }
        }

        // ── With Labels ─────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "With Labels" }
            <SwitchRow> {
                <View> {
                    width: Fit, height: Fit, flow: Right, spacing: 8, align: { y: 0.5 }
                    <Switch> {}
                    <Label> {
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                            color: (COLOR_TEXT)
                        }
                        text: "Notifications"
                    }
                }
                <View> {
                    width: Fit, height: Fit, flow: Right, spacing: 8, align: { y: 0.5 }
                    <DangerSwitch> { active: true }
                    <Label> {
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                            color: (COLOR_TEXT)
                        }
                        text: "Delete on exit"
                    }
                }
            }
        }
    }
}
