#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApexAction {
    id: String,
    label: Option<String>,
}

impl ApexAction {
    pub fn named(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: None,
        }
    }

    pub fn labeled(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: Some(label.into()),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::ApexAction;

    #[test]
    fn named_action_keeps_stable_id() {
        let action = ApexAction::named("create");

        assert_eq!(action.id(), "create");
        assert_eq!(action.label(), None);
    }

    #[test]
    fn labeled_action_keeps_user_label_separate_from_id() {
        let action = ApexAction::labeled("create_asset", "Create Asset");

        assert_eq!(action.id(), "create_asset");
        assert_eq!(action.label(), Some("Create Asset"));
    }
}
