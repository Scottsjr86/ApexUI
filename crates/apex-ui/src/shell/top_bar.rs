use crate::{theme::ApexTheme, ui::ApexUi};

pub(crate) fn show(
    ui: &mut eframe::egui::Ui,
    theme: &ApexTheme,
    body: impl FnOnce(&mut ApexUi<'_>),
) {
    let frame = eframe::egui::Frame::new()
        .fill(theme.colors.top_bar.to_egui())
        .stroke(eframe::egui::Stroke::new(
            1.0,
            theme.colors.border.to_egui(),
        ))
        .corner_radius(eframe::egui::CornerRadius::same(theme.radius.lg))
        .inner_margin(eframe::egui::Margin::symmetric(
            theme.spacing.md as i8,
            theme.spacing.sm as i8,
        ));

    frame.show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = theme.spacing.sm;
            let mut apex_ui = ApexUi::new(ui, theme);
            body(&mut apex_ui);
        });
    });
    ui.add_space(theme.spacing.md);
}
