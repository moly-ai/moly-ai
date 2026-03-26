use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use moly_widgets::theme::*;
    use moly_widgets::text_input::*;

    SectionLabel = <Label> {
        width: Fill,
        padding: { bottom: 4 }
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 11 }
            color: (COLOR_TEXT)
        }
    }

    InputPair = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 16,
        align: { y: 0.5 }
    }

    InputLabel = <Label> {
        width: 70,
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 9 }
            color: (COLOR_DISABLED_TEXT)
        }
    }

    pub TextInputShowcase = <View> {
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
            text: "Text Inputs"
        }

        // ── Primary ──────────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Primary" }
            <InputPair> {
                <InputLabel> { text: "Normal" }
                <View> {
                    width: 300, height: Fit,
                    <PrimaryTextInput> {
                        empty_text: "Type something..."
                    }
                }
            }
            <InputPair> {
                <InputLabel> { text: "Disabled" }
                <View> {
                    width: 300, height: Fit,
                    disabled_primary = <PrimaryTextInput> {
                        empty_text: "Disabled"
                        is_read_only: true,
                    }
                }
            }
        }

        // ── Neutral ──────────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Neutral" }
            <InputPair> {
                <InputLabel> { text: "Normal" }
                <View> {
                    width: 300, height: Fit,
                    <NeutralTextInput> {
                        empty_text: "Type something..."
                    }
                }
            }
            <InputPair> {
                <InputLabel> { text: "Disabled" }
                <View> {
                    width: 300, height: Fit,
                    disabled_neutral = <NeutralTextInput> {
                        empty_text: "Disabled"
                        is_read_only: true,
                    }
                }
            }
        }

        // ── Danger ───────────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Danger" }
            <InputPair> {
                <InputLabel> { text: "Normal" }
                <View> {
                    width: 300, height: Fit,
                    <DangerTextInput> {
                        empty_text: "Type something..."
                    }
                }
            }
            <InputPair> {
                <InputLabel> { text: "Disabled" }
                <View> {
                    width: 300, height: Fit,
                    disabled_danger = <DangerTextInput> {
                        empty_text: "Disabled"
                        is_read_only: true,
                    }
                }
            }
        }

        // ── Warning ──────────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Warning" }
            <InputPair> {
                <InputLabel> { text: "Normal" }
                <View> {
                    width: 300, height: Fit,
                    <WarningTextInput> {
                        empty_text: "Type something..."
                    }
                }
            }
            <InputPair> {
                <InputLabel> { text: "Disabled" }
                <View> {
                    width: 300, height: Fit,
                    disabled_warning = <WarningTextInput> {
                        empty_text: "Disabled"
                        is_read_only: true,
                    }
                }
            }
        }

        // ── Transparent ──────────────────────────────────────────
        <View> {
            width: Fill, height: Fit, flow: Down, spacing: 8,
            <SectionLabel> { text: "Transparent" }
            <InputPair> {
                <InputLabel> { text: "Normal" }
                <View> {
                    width: 300, height: Fit,
                    <TransparentTextInput> {
                        empty_text: "No border, no background..."
                    }
                }
            }
            <InputPair> {
                <InputLabel> { text: "Disabled" }
                <View> {
                    width: 300, height: Fit,
                    disabled_transparent = <TransparentTextInput> {
                        empty_text: "Disabled"
                        is_read_only: true,
                    }
                }
            }
        }
    }
}
