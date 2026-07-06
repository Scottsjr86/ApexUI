use apex_ui::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DemoState {
    counter: usize,
    last_action: Option<String>,
}

impl DemoState {
    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn last_action(&self) -> Option<&str> {
        self.last_action.as_deref()
    }

    pub fn apply_action(&mut self, action: &ApexAction) {
        match action.id() {
            "increment" => self.counter += 1,
            "reset" => self.counter = 0,
            "create" => {}
            _ => {}
        }

        self.last_action = Some(action.id().to_owned());
    }

    #[cfg(test)]
    pub fn semantic_snapshot(&self) -> String {
        phase01_demo_semantics(self.counter, self.last_action())
    }

    #[cfg(test)]
    pub fn behavior_snapshot(&self) -> String {
        format!(
            "behavior:phase01-demo-counter\ncounter={}\nlast_action={}\n",
            self.counter,
            self.last_action().unwrap_or("none")
        )
    }
}

#[cfg(test)]
mod tests {
    use apex_ui::prelude::ApexAction;

    use super::DemoState;

    #[test]
    fn demo_state_records_counter_path_as_behavior_golden() {
        let mut state = DemoState::default();
        state.apply_action(&ApexAction::named("increment"));
        state.apply_action(&ApexAction::named("increment"));
        state.apply_action(&ApexAction::named("reset"));

        let golden = include_str!("../../../tests/goldens/behavior/phase01_demo_counter.golden");
        assert_eq!(state.behavior_snapshot(), golden);
    }

    #[test]
    fn demo_state_keeps_create_as_action_without_fake_counter_change() {
        let mut state = DemoState::default();
        state.apply_action(&ApexAction::named("create"));

        assert_eq!(state.counter(), 0);
        assert_eq!(state.last_action(), Some("create"));
    }

    #[test]
    fn demo_state_exports_current_semantics() {
        let state = DemoState::default();

        assert!(state.semantic_snapshot().contains("counter=0"));
        assert!(state.semantic_snapshot().contains("last_action=none"));
    }
}
