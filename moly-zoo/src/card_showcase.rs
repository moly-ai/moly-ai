use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use moly_widgets::theme::*;
    use moly_widgets::card::*;

    pub CardShowcase = <View> {
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
            text: "Cards"
        }

        // ── Default Card ────────────────────────────────────────
        <Card> {
            width: 300,
            <Label> {
                width: Fill,
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 11 }
                    color: (COLOR_TEXT)
                    wrap: Word
                }
                text: "Default Card"
            }
            <Label> {
                width: Fill,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                    color: (COLOR_TEXT)
                    wrap: Word
                }
                text: "This is a simple elevated card with surface color, rounded corners, and a subtle shadow."
            }
        }

        // ── Card with custom content ────────────────────────────
        <Card> {
            width: 300,
            spacing: 8,
            <Label> {
                width: Fill,
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 11 }
                    color: (COLOR_TEXT)
                    wrap: Word
                }
                text: "Card with more content"
            }
            <Label> {
                width: Fill,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                    color: (COLOR_TEXT)
                    wrap: Word
                }
                text: "Cards can hold any child widgets. They provide visual grouping through elevation."
            }
            <View> {
                width: Fill, height: 1,
                show_bg: true,
                draw_bg: { color: (COLOR_BORDER) }
            }
            <Label> {
                width: Fill,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                    color: (COLOR_DISABLED_TEXT)
                    wrap: Word
                }
                text: "Footer text"
            }
        }

        // ── Fill-width Card ─────────────────────────────────────
        <Card> {
            width: Fill,
            <Label> {
                width: Fill,
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 11 }
                    color: (COLOR_TEXT)
                    wrap: Word
                }
                text: "Full-width Card"
            }
            <Label> {
                width: Fill,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                    color: (COLOR_TEXT)
                    wrap: Word
                }
                text: "This card stretches to fill the available width."
            }
        }
    }
}
