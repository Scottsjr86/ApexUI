use crate::response::ApexResponse;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ApexSemanticSnapshot {
    lines: Vec<String>,
}

impl ApexSemanticSnapshot {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            lines: vec![format!("snapshot:{}", name.into())],
        }
    }

    pub fn push(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.lines
            .push(format!("{}={}", key.as_ref(), value.as_ref()));
        self
    }

    pub fn render(&self) -> String {
        let mut rendered = self.lines.join("\n");
        rendered.push('\n');
        rendered
    }
}

pub fn phase01_demo_semantics(counter: usize, last_action: Option<&str>) -> String {
    ApexSemanticSnapshot::new("phase01-demo-shell")
        .push("shell", "Apex Demo")
        .push("top_bar", "Create Reset Settings")
        .push("disabled_action", "Settings")
        .push("panel", "Core Forge")
        .push("section", "Apex facade is online")
        .push("card", "Apex API surface")
        .push("primary_button", "Increment")
        .push("status_chip", "Starter shell online")
        .push("status_text", "Apex APIs drive the demo path")
        .push("counter", counter.to_string())
        .push("last_action", last_action.unwrap_or("none"))
        .render()
}

pub fn phase01_interaction_state_semantics() -> String {
    let disabled = ApexResponse::new(true, true, true, true, false);
    let pressed = ApexResponse::new(false, true, true, true, true);
    let hovered = ApexResponse::new(false, true, false, true, true);
    let focused = ApexResponse::new(false, false, false, true, true);
    let idle = ApexResponse::new(false, false, false, false, true);

    ApexSemanticSnapshot::new("phase01-interaction-states")
        .push("disabled", disabled.interaction_state().as_str())
        .push("pressed_priority", pressed.interaction_state().as_str())
        .push("hovered_priority", hovered.interaction_state().as_str())
        .push("focused_priority", focused.interaction_state().as_str())
        .push("idle", idle.interaction_state().as_str())
        .render()
}

#[cfg(test)]
mod tests {
    use super::{phase01_demo_semantics, phase01_interaction_state_semantics};

    #[test]
    fn phase01_semantic_snapshot_matches_accepted_demo_shell_golden() {
        let snapshot = phase01_demo_semantics(0, None);
        let golden = include_str!("../../../../tests/goldens/semantic/phase01_demo_shell.golden");

        assert_eq!(snapshot, golden);
    }

    #[test]
    fn phase01_semantic_snapshot_tracks_counter_and_action() {
        let snapshot = phase01_demo_semantics(3, Some("increment"));

        assert!(snapshot.contains("snapshot:phase01-demo-shell"));
        assert!(snapshot.contains("counter=3"));
        assert!(snapshot.contains("last_action=increment"));
    }

    #[test]
    fn phase01_semantic_snapshot_records_disabled_settings_action() {
        let snapshot = phase01_demo_semantics(0, None);

        assert!(snapshot.contains("top_bar=Create Reset Settings"));
        assert!(snapshot.contains("disabled_action=Settings"));
    }

    #[test]
    fn phase01_interaction_state_semantic_snapshot_matches_golden() {
        let snapshot = phase01_interaction_state_semantics();
        let golden =
            include_str!("../../../../tests/goldens/semantic/phase01_interaction_states.golden");

        assert_eq!(snapshot, golden);
    }
}
