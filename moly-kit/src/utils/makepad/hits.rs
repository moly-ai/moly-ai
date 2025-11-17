use makepad_widgets::*;

pub trait HitExt {
    /// If the primary pointer action happened, returns the position where it happened.
    fn primary_pointer_action_pos(&self) -> Option<Vec2>;
    /// If the secondary pointer action happened, returns the position where it happened.
    fn secondary_pointer_action_pos(&self) -> Option<Vec2>;
    /// This was a left mouse click or a simple touch screen tap.
    fn is_primary_pointer_action(&self) -> bool;
    /// This was a right mouse click or a long press on touch screen.
    fn is_secondary_pointer_action(&self) -> bool;
}

impl HitExt for Hit {
    fn primary_pointer_action_pos(&self) -> Option<Vec2> {
        match self {
            Hit::FingerUp(fu)
                if fu.was_tap()
                    && ((fu.is_mouse() && fu.mouse_button().unwrap().is_primary())
                        || fu.is_touch()) =>
            {
                Some(fu.abs.into_vec2())
            }
            _ => None,
        }
    }

    fn secondary_pointer_action_pos(&self) -> Option<Vec2> {
        match self {
            Hit::FingerUp(fu)
                if fu.was_tap() && fu.is_mouse() && fu.mouse_button().unwrap().is_secondary() =>
            {
                Some(fu.abs.into_vec2())
            }
            Hit::FingerLongPress(flp) => Some(flp.abs.into_vec2()),
            _ => None,
        }
    }

    fn is_primary_pointer_action(&self) -> bool {
        self.primary_pointer_action_pos().is_some()
    }

    fn is_secondary_pointer_action(&self) -> bool {
        self.secondary_pointer_action_pos().is_some()
    }
}
