use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use moly_widgets::theme::*;
    use moly_widgets::button::*;

    ICON_CLOSE = dep("crate://self/resources/icons/close.svg")

    SectionLabel = <Label> {
        width: Fill,
        padding: { bottom: 4 }
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 11 }
            color: (COLOR_TEXT)
        }
    }

    ButtonRow = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 12,
        align: { y: 0.5 }
    }

    pub ButtonShowcase = <View> {
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
            text: "Buttons"
        }

        // ── Filled ───────────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Filled" }
            <ButtonRow> {
                <FilledButton> { text: "Primary" }
                <FilledDangerButton> { text: "Danger" }
                <FilledWarningButton> { text: "Warning" }
                <FilledNeutralButton> { text: "Neutral" }
            }
        }

        // ── Outlined ─────────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Outlined" }
            <ButtonRow> {
                <OutlinedButton> { text: "Primary" }
                <OutlinedDangerButton> { text: "Danger" }
                <OutlinedWarningButton> { text: "Warning" }
                <OutlinedNeutralButton> { text: "Neutral" }
            }
        }

        // ── Text ─────────────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Text" }
            <ButtonRow> {
                <TextButton> { text: "Primary" }
                <TextDangerButton> { text: "Danger" }
                <TextWarningButton> { text: "Warning" }
                <TextNeutralButton> { text: "Neutral" }
            }
        }

        // ── Circular Icon Buttons ────────────────────────────────
        // Makepad's border_radius is absolute (pixels), not relative
        // to the widget size. To make a perfect circle we must set
        // width & height to fixed equal values and border_radius to
        // a specific value that works for that size. The text uses Font Awesome
        // (THEME_FONT_ICONS) so a Unicode codepoint renders as an icon glyph.
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Circular Icon (Font Awesome)" }
            <ButtonRow> {
                <FilledButton> {
                    width: 40, height: 40,
                    padding: 0,
                    align: { x: 0.5, y: 0.5 }
                    // 1/4 of the size seems to work for a perfect circle.
                    draw_bg: { uniform border_radius: 10.0 }
                    draw_text: {
                        text_style: <THEME_FONT_ICONS> { font_size: 14 }
                    }
                    text: ""
                }
                <OutlinedButton> {
                    width: 40, height: 40,
                    padding: 0,
                    align: { x: 0.5, y: 0.5 }
                    draw_bg: { uniform border_radius: 10.0 }
                    draw_text: {
                        text_style: <THEME_FONT_ICONS> { font_size: 14 }
                    }
                    text: ""
                }
                <TextButton> {
                    width: 40, height: 40,
                    padding: 0,
                    align: { x: 0.5, y: 0.5 }
                    draw_bg: { uniform border_radius: 10.0 }
                    draw_text: {
                        text_style: <THEME_FONT_ICONS> { font_size: 14 }
                    }
                    text: ""
                }
            }
        }

        // ── Circular Icon Buttons (SVG) ──────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Circular Icon (SVG - draw_icon)" }
            <ButtonRow> {
                <FilledButton> {
                    width: 40, height: 40,
                    padding: 0,
                    spacing: 0,
                    align: { x: 0.5, y: 0.5 }
                    draw_bg: { uniform border_radius: 10.0 }
                    icon_walk: { width: 14, height: 14 }
                    draw_icon: { svg_file: (ICON_CLOSE) }
                    text: ""
                }
                <OutlinedButton> {
                    width: 40, height: 40,
                    padding: 0,
                    spacing: 0,
                    align: { x: 0.5, y: 0.5 }
                    draw_bg: { uniform border_radius: 10.0 }
                    icon_walk: { width: 14, height: 14 }
                    draw_icon: { svg_file: (ICON_CLOSE) }
                    text: ""
                }
                <TextButton> {
                    width: 40, height: 40,
                    padding: 0,
                    spacing: 0,
                    align: { x: 0.5, y: 0.5 }
                    draw_bg: { uniform border_radius: 10.0 }
                    icon_walk: { width: 14, height: 14 }
                    draw_icon: { svg_file: (ICON_CLOSE) }
                    text: ""
                }
            }
        }

        // ── SVG Icon Buttons ─────────────────────────────────────
        // Buttons with SVG icons via draw_icon. The svg_file is loaded
        // with dep() and icon_walk controls the icon size.
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "SVG Icon (draw_icon)" }
            <ButtonRow> {
                // Icon + text
                <FilledButton> {
                    icon_walk: { width: 14, height: 14 }
                    draw_icon: {
                        svg_file: (ICON_CLOSE),
                    }
                    text: "Close"
                }
                // Icon only (no text): spacing must be 0 so the
                // layout doesn't reserve a gap for the empty label.
                <FilledButton> {
                    width: 40, height: 40,
                    padding: 0,
                    spacing: 0,
                    align: { x: 0.5, y: 0.5 }
                    icon_walk: { width: 14, height: 14 }
                    draw_icon: {
                        svg_file: (ICON_CLOSE),
                    }
                    text: ""
                }
            }
        }
    }
}
