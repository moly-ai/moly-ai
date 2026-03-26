use makepad_widgets::*;

live_design! {
    use link::theme::*;

    // ── Base Colors ──────────────────────────────────────────────────────

    pub COLOR_PRIMARY = #0D9488
    pub COLOR_DANGER = #DC3545
    pub COLOR_WARNING = #F59E0B
    pub COLOR_NEUTRAL = #1A1A1A

    // ── Derived: Hover & Down (mix with white for brightness) ────────────

    pub COLOR_PRIMARY_HOVER = (mix(COLOR_PRIMARY, #FFF, 0.2))
    pub COLOR_PRIMARY_DOWN = (mix(COLOR_PRIMARY, #FFF, 0.35))

    pub COLOR_DANGER_HOVER = (mix(COLOR_DANGER, #FFF, 0.2))
    pub COLOR_DANGER_DOWN = (mix(COLOR_DANGER, #FFF, 0.35))

    pub COLOR_WARNING_HOVER = (mix(COLOR_WARNING, #FFF, 0.2))
    pub COLOR_WARNING_DOWN = (mix(COLOR_WARNING, #FFF, 0.35))

    pub COLOR_NEUTRAL_HOVER = (mix(COLOR_NEUTRAL, #FFF, 0.2))
    pub COLOR_NEUTRAL_DOWN = (mix(COLOR_NEUTRAL, #FFF, 0.35))

    // ── Muted Tints (base color at low alpha, for outlined/text hover) ──
    // Cannot derive alpha with mix(), so these are hardcoded.
    // Pattern: base color RGB + ~9% alpha (18 hex) / ~16% alpha (28 hex).

    pub COLOR_PRIMARY_MUTED = #0D948818
    pub COLOR_PRIMARY_MUTED_HOVER = #0D948828

    pub COLOR_DANGER_MUTED = #DC354518
    pub COLOR_DANGER_MUTED_HOVER = #DC354528

    pub COLOR_WARNING_MUTED = #F59E0B18
    pub COLOR_WARNING_MUTED_HOVER = #F59E0B28

    pub COLOR_NEUTRAL_MUTED = #1A1A1A18
    pub COLOR_NEUTRAL_MUTED_HOVER = #1A1A1A28

    // ── Surfaces & Backgrounds ───────────────────────────────────────────

    pub COLOR_BG = #F5F5F5
    pub COLOR_SURFACE = #FFFFFF
    pub COLOR_BORDER = #E0E0E0

    // ── Text ─────────────────────────────────────────────────────────────

    pub COLOR_TEXT = #1A1A1A
    pub COLOR_TEXT_ON_FILLED = #FFFFFF

    // ── Disabled ─────────────────────────────────────────────────────────

    pub COLOR_DISABLED_BG = #F0F0F0
    pub COLOR_DISABLED_TEXT = #9CA3AF
    pub COLOR_DISABLED_BORDER = #E5E5E5

    // ── Utility ──────────────────────────────────────────────────────────

    pub COLOR_TRANSPARENT = #0000

    // ── Radii ────────────────────────────────────────────────────────────

    pub RADIUS_BUTTON = 4.0
    pub RADIUS_CARD = 6.0
    pub RADIUS_INPUT = 4.0
}
