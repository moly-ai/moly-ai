//! Extensions to Makepad's `Hit` type. Useful place to detect common UX patterns.

use makepad_widgets::*;

pub trait HitExt {
    /// If the primary pointer action happened, returns the position where it happened.
    fn primary_pointer_action_pos(&self) -> Option<DVec2>;
    /// If the secondary pointer action happened, returns the position where it happened.
    fn secondary_pointer_action_pos(&self) -> Option<DVec2>;
    /// This was a left mouse click or a simple touch screen tap.
    fn is_primary_pointer_action(&self) -> bool;
    /// This was a right mouse click or a long press on touch screen.
    fn is_secondary_pointer_action(&self) -> bool;
}

impl HitExt for Hit {
    fn primary_pointer_action_pos(&self) -> Option<DVec2> {
        match self {
            Hit::FingerUp(fu)
                if fu.was_tap()
                    && ((fu.is_mouse() && fu.mouse_button().unwrap().is_primary())
                        || fu.is_touch()) =>
            {
                Some(fu.abs)
            }
            _ => None,
        }
    }

    fn secondary_pointer_action_pos(&self) -> Option<DVec2> {
        match self {
            Hit::FingerUp(fu)
                if fu.was_tap() && fu.is_mouse() && fu.mouse_button().unwrap().is_secondary() =>
            {
                Some(fu.abs)
            }
            Hit::FingerLongPress(flp) => Some(flp.abs),
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
