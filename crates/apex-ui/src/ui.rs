use crate::{
    action::ApexAction,
    response::ApexResponse,
    theme::ApexTheme,
    widgets::{button, panel, status_chip},
};

pub struct ApexUi<'a> {
    pub(crate) egui_ui: &'a mut eframe::egui::Ui,
    pub(crate) theme: &'a ApexTheme,
}

impl<'a> ApexUi<'a> {
    pub(crate) fn new(egui_ui: &'a mut eframe::egui::Ui, theme: &'a ApexTheme) -> Self {
        Self { egui_ui, theme }
    }

    pub fn top_bar(&mut self, body: impl FnOnce(&mut ApexUi<'_>)) {
        crate::shell::top_bar::show(self.egui_ui, self.theme, body);
    }

    pub fn status_bar(&mut self, body: impl FnOnce(&mut ApexUi<'_>)) {
        crate::shell::status_bar::show(self.egui_ui, self.theme, body);
    }

    pub fn panel(&mut self, title: impl Into<String>, body: impl FnOnce(&mut ApexUi<'_>)) {
        panel::show_panel(self.egui_ui, self.theme, title.into(), body);
    }

    pub fn card(&mut self, body: impl FnOnce(&mut ApexUi<'_>)) {
        panel::show_card(self.egui_ui, self.theme, body);
    }

    pub fn section_header(&mut self, text: impl AsRef<str>) {
        panel::section_header(self.egui_ui, self.theme, text.as_ref());
    }

    pub fn body_text(&mut self, text: impl AsRef<str>) {
        panel::body_text(self.egui_ui, self.theme, text.as_ref());
    }

    pub fn status_text(&mut self, text: impl AsRef<str>) {
        status_chip::status_text(self.egui_ui, self.theme, text.as_ref());
    }

    pub fn status_chip(&mut self, text: impl AsRef<str>) {
        status_chip::status_chip(self.egui_ui, self.theme, text.as_ref());
    }

    pub fn primary_button(&mut self, label: impl AsRef<str>) -> ApexResponse {
        button::button(
            self.egui_ui,
            self.theme,
            label.as_ref(),
            button::ButtonKind::Primary,
            true,
        )
    }

    pub fn quiet_button(&mut self, label: impl AsRef<str>) -> ApexResponse {
        button::button(
            self.egui_ui,
            self.theme,
            label.as_ref(),
            button::ButtonKind::Quiet,
            true,
        )
    }

    pub fn disabled_primary_button(&mut self, label: impl AsRef<str>) -> ApexResponse {
        button::button(
            self.egui_ui,
            self.theme,
            label.as_ref(),
            button::ButtonKind::Primary,
            false,
        )
    }

    pub fn disabled_quiet_button(&mut self, label: impl AsRef<str>) -> ApexResponse {
        button::button(
            self.egui_ui,
            self.theme,
            label.as_ref(),
            button::ButtonKind::Quiet,
            false,
        )
    }

    pub fn primary_action(&mut self, label: impl AsRef<str>, action: ApexAction) -> ApexResponse {
        self.primary_button(label).on_click(action)
    }

    pub fn quiet_action(&mut self, label: impl AsRef<str>, action: ApexAction) -> ApexResponse {
        self.quiet_button(label).on_click(action)
    }

    pub fn disabled_primary_action(
        &mut self,
        label: impl AsRef<str>,
        action: ApexAction,
    ) -> ApexResponse {
        self.disabled_primary_button(label).on_click(action)
    }

    pub fn disabled_quiet_action(
        &mut self,
        label: impl AsRef<str>,
        action: ApexAction,
    ) -> ApexResponse {
        self.disabled_quiet_button(label).on_click(action)
    }
}
