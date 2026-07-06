use crate::{theme::ApexTheme, ui::ApexUi};

pub(crate) fn show(
    ui: &mut eframe::egui::Ui,
    theme: &ApexTheme,
    title: String,
    body: impl FnOnce(&mut ApexUi<'_>),
) {
    let frame = eframe::egui::Frame::new()
        .fill(theme.colors.app_background.to_egui())
        .inner_margin(eframe::egui::Margin::same(theme.spacing.window as i8));

    frame.show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        title_row(ui, theme, &title);
        ui.add_space(theme.spacing.sm);

        let mut apex_ui = ApexUi::new(ui, theme);
        body(&mut apex_ui);
    });
}

fn title_row(ui: &mut eframe::egui::Ui, theme: &ApexTheme, title: &str) {
    ui.horizontal(|ui| {
        ui.painter().text(
            ui.cursor().left_top() + eframe::egui::vec2(0.0, 3.0),
            eframe::egui::Align2::LEFT_TOP,
            title,
            eframe::egui::FontId::proportional(theme.typography.section + 2.0),
            theme.colors.text.to_egui(),
        );
        ui.add_space((title.chars().count() as f32 * 10.0).max(120.0));
    });
}
