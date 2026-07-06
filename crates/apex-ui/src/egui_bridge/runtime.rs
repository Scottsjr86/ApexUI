use crate::{
    app::ApexApp, egui_bridge::style::apply_theme_to_context, frame::ApexFrame, theme::ApexTheme,
};

pub(crate) struct EframeApexAdapter<A> {
    app: A,
    theme: ApexTheme,
}

impl<A: ApexApp> EframeApexAdapter<A> {
    pub(crate) fn new(app: A, cc: &eframe::CreationContext<'_>) -> Self {
        let theme = ApexTheme::default();
        apply_theme_to_context(&cc.egui_ctx, &theme);

        Self { app, theme }
    }
}

impl<A: ApexApp> eframe::App for EframeApexAdapter<A> {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        apply_theme_to_context(ui.ctx(), &self.theme);

        let mut frame = ApexFrame::new(ui, &self.theme);
        self.app.update(&mut frame);
    }
}
