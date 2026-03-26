use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use moly_widgets::theme::*;

    ColorSwatch = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 12,
        align: { y: 0.5 }
        padding: { top: 4, bottom: 4 }

        swatch = <RoundedView> {
            width: 32,
            height: 32,
            draw_bg: {
                border_radius: 4.0,
                color: #000,
            }
        }

        label = <Label> {
            width: Fill,
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                color: (COLOR_TEXT)
            }
        }
    }

    SectionTitle = <Label> {
        width: Fill,
        padding: { top: 12, bottom: 4 }
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 11 }
            color: (COLOR_TEXT)
        }
    }

    pub ThemeShowcase = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 4,
        padding: 32,

        <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 14 }
                color: (COLOR_TEXT)
            }
            text: "Theme Colors"
        }

        // ── Base Colors ─────────────────────────────────────────
        <SectionTitle> { text: "Base Colors" }

        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_PRIMARY) } }
            label = { text: "COLOR_PRIMARY" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_DANGER) } }
            label = { text: "COLOR_DANGER" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_WARNING) } }
            label = { text: "COLOR_WARNING" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_NEUTRAL) } }
            label = { text: "COLOR_NEUTRAL" }
        }

        // ── Hover & Down ────────────────────────────────────────
        <SectionTitle> { text: "Hover & Down" }

        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_PRIMARY_HOVER) } }
            label = { text: "COLOR_PRIMARY_HOVER" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_PRIMARY_DOWN) } }
            label = { text: "COLOR_PRIMARY_DOWN" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_DANGER_HOVER) } }
            label = { text: "COLOR_DANGER_HOVER" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_DANGER_DOWN) } }
            label = { text: "COLOR_DANGER_DOWN" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_WARNING_HOVER) } }
            label = { text: "COLOR_WARNING_HOVER" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_WARNING_DOWN) } }
            label = { text: "COLOR_WARNING_DOWN" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_NEUTRAL_HOVER) } }
            label = { text: "COLOR_NEUTRAL_HOVER" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_NEUTRAL_DOWN) } }
            label = { text: "COLOR_NEUTRAL_DOWN" }
        }

        // ── Muted Tints ─────────────────────────────────────────
        <SectionTitle> { text: "Muted Tints" }

        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_PRIMARY_MUTED) } }
            label = { text: "COLOR_PRIMARY_MUTED" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_PRIMARY_MUTED_HOVER) } }
            label = { text: "COLOR_PRIMARY_MUTED_HOVER" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_DANGER_MUTED) } }
            label = { text: "COLOR_DANGER_MUTED" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_DANGER_MUTED_HOVER) } }
            label = { text: "COLOR_DANGER_MUTED_HOVER" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_WARNING_MUTED) } }
            label = { text: "COLOR_WARNING_MUTED" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_WARNING_MUTED_HOVER) } }
            label = { text: "COLOR_WARNING_MUTED_HOVER" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_NEUTRAL_MUTED) } }
            label = { text: "COLOR_NEUTRAL_MUTED" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_NEUTRAL_MUTED_HOVER) } }
            label = { text: "COLOR_NEUTRAL_MUTED_HOVER" }
        }

        // ── Surfaces & Backgrounds ──────────────────────────────
        <SectionTitle> { text: "Surfaces & Backgrounds" }

        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_BG) } }
            label = { text: "COLOR_BG" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_SURFACE) } }
            label = { text: "COLOR_SURFACE" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_BORDER) } }
            label = { text: "COLOR_BORDER" }
        }

        // ── Text ────────────────────────────────────────────────
        <SectionTitle> { text: "Text" }

        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_TEXT) } }
            label = { text: "COLOR_TEXT" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_TEXT_ON_FILLED) } }
            label = { text: "COLOR_TEXT_ON_FILLED" }
        }

        // ── Disabled ────────────────────────────────────────────
        <SectionTitle> { text: "Disabled" }

        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_DISABLED_BG) } }
            label = { text: "COLOR_DISABLED_BG" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_DISABLED_TEXT) } }
            label = { text: "COLOR_DISABLED_TEXT" }
        }
        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_DISABLED_BORDER) } }
            label = { text: "COLOR_DISABLED_BORDER" }
        }

        // ── Utility ─────────────────────────────────────────────
        <SectionTitle> { text: "Utility" }

        <ColorSwatch> {
            swatch = { draw_bg: { color: (COLOR_TRANSPARENT) } }
            label = { text: "COLOR_TRANSPARENT" }
        }
    }
}
