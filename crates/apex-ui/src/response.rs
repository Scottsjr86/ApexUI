use crate::action::ApexAction;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ApexInteractionState {
    Disabled,
    Pressed,
    Hovered,
    Focused,
    #[default]
    Idle,
}

impl ApexInteractionState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Pressed => "pressed",
            Self::Hovered => "hovered",
            Self::Focused => "focused",
            Self::Idle => "idle",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ApexResponse {
    activated: bool,
    hovered: bool,
    pressed: bool,
    focused: bool,
    enabled: bool,
    action: Option<ApexAction>,
}

impl ApexResponse {
    pub fn new(
        activated: bool,
        hovered: bool,
        pressed: bool,
        focused: bool,
        enabled: bool,
    ) -> Self {
        Self {
            activated: enabled && activated,
            hovered,
            pressed: enabled && pressed,
            focused,
            enabled,
            action: None,
        }
    }

    pub fn activated(&self) -> bool {
        self.activated
    }

    pub fn hovered(&self) -> bool {
        self.hovered
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn interaction_state(&self) -> ApexInteractionState {
        if !self.enabled {
            return ApexInteractionState::Disabled;
        }

        if self.pressed {
            return ApexInteractionState::Pressed;
        }

        if self.hovered {
            return ApexInteractionState::Hovered;
        }

        if self.focused {
            return ApexInteractionState::Focused;
        }

        ApexInteractionState::Idle
    }

    pub fn on_click(mut self, action: ApexAction) -> Self {
        if self.enabled && self.activated {
            self.action = Some(action);
        }
        self
    }

    pub fn action(&self) -> Option<&ApexAction> {
        self.action.as_ref()
    }

    pub fn take_action(self) -> Option<ApexAction> {
        self.action
    }

    pub(crate) fn from_egui(response: &eframe::egui::Response, enabled: bool) -> Self {
        Self::new(
            enabled && response.clicked(),
            response.hovered(),
            enabled && response.is_pointer_button_down_on(),
            response.has_focus(),
            enabled,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::action::ApexAction;

    use super::{ApexInteractionState, ApexResponse};

    #[test]
    fn on_click_emits_action_only_when_activated_and_enabled() {
        let active = ApexResponse::new(true, true, false, false, true)
            .on_click(ApexAction::named("increment"));
        let quiet = ApexResponse::new(false, false, false, false, true)
            .on_click(ApexAction::named("increment"));
        let disabled = ApexResponse::new(true, true, false, false, false)
            .on_click(ApexAction::named("increment"));

        assert_eq!(active.action().map(ApexAction::id), Some("increment"));
        assert_eq!(quiet.action(), None);
        assert_eq!(disabled.action(), None);
    }

    #[test]
    fn response_keeps_apex_interaction_language() {
        let response = ApexResponse::new(false, true, true, false, false);

        assert!(!response.activated());
        assert!(response.hovered());
        assert!(!response.pressed());
        assert!(!response.focused());
        assert!(!response.enabled());
        assert_eq!(response.interaction_state(), ApexInteractionState::Disabled);
    }

    #[test]
    fn interaction_state_names_are_stable_for_snapshots() {
        assert_eq!(ApexInteractionState::Disabled.as_str(), "disabled");
        assert_eq!(ApexInteractionState::Pressed.as_str(), "pressed");
        assert_eq!(ApexInteractionState::Hovered.as_str(), "hovered");
        assert_eq!(ApexInteractionState::Focused.as_str(), "focused");
        assert_eq!(ApexInteractionState::Idle.as_str(), "idle");
    }

    #[test]
    fn interaction_state_has_apex_priority_order() {
        let disabled = ApexResponse::new(true, true, true, true, false);
        let pressed = ApexResponse::new(false, true, true, true, true);
        let hovered = ApexResponse::new(false, true, false, true, true);
        let focused = ApexResponse::new(false, false, false, true, true);
        let idle = ApexResponse::new(false, false, false, false, true);

        assert_eq!(disabled.interaction_state(), ApexInteractionState::Disabled);
        assert_eq!(pressed.interaction_state(), ApexInteractionState::Pressed);
        assert_eq!(hovered.interaction_state(), ApexInteractionState::Hovered);
        assert_eq!(focused.interaction_state(), ApexInteractionState::Focused);
        assert_eq!(idle.interaction_state(), ApexInteractionState::Idle);
    }
}
