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

#[cfg(test)]
mod tests {
    use super::phase01_demo_semantics;

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
}
