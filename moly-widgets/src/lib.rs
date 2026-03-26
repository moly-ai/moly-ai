use makepad_widgets::Cx;

pub mod button;
pub mod card;
pub mod sidebar;
pub mod stack_router;
pub mod switch;
pub mod text_input;
pub mod theme;

pub fn live_design(cx: &mut Cx) {
    theme::live_design(cx);
    button::live_design(cx);
    card::live_design(cx);
    switch::live_design(cx);
    text_input::live_design(cx);
}
