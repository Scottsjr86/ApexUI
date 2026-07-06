use crate::action::ApexAction;

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

    use super::ApexResponse;

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
    }
}
