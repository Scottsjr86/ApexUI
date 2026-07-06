use crate::{shell, theme::ApexTheme, ui::ApexUi};

pub struct ApexFrame<'a> {
    egui_ui: &'a mut eframe::egui::Ui,
    theme: &'a ApexTheme,
}

impl<'a> ApexFrame<'a> {
    pub(crate) fn new(egui_ui: &'a mut eframe::egui::Ui, theme: &'a ApexTheme) -> Self {
        Self { egui_ui, theme }
    }

    pub fn shell(&mut self, title: impl Into<String>, body: impl FnOnce(&mut ApexUi<'_>)) {
        shell::app_shell::show(self.egui_ui, self.theme, title.into(), body);
    }

    pub fn raw_egui(&mut self, body: impl FnOnce(&mut eframe::egui::Ui)) {
        body(self.egui_ui);
    }
}
