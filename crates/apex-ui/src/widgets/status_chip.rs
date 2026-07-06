use crate::theme::ApexTheme;

pub(crate) fn status_chip(ui: &mut eframe::egui::Ui, theme: &ApexTheme, text: &str) {
    let width = (text.chars().count() as f32 * 7.0 + theme.spacing.md * 2.0).max(74.0);
    let height = theme.typography.status + theme.spacing.sm;
    let (rect, _) = ui.allocate_exact_size(
        eframe::egui::vec2(width, height),
        eframe::egui::Sense::hover(),
    );

    ui.painter().rect(
        rect,
        eframe::egui::CornerRadius::same(theme.radius.pill),
        theme.colors.quiet_hover.to_egui(),
        eframe::egui::Stroke::new(1.0, theme.colors.border.to_egui()),
        eframe::egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        eframe::egui::Align2::CENTER_CENTER,
        text,
        eframe::egui::FontId::proportional(theme.typography.status),
        theme.colors.text.to_egui(),
    );
}

pub(crate) fn status_text(ui: &mut eframe::egui::Ui, theme: &ApexTheme, text: &str) {
    let width = (text.chars().count() as f32 * 7.0).max(80.0);
    let height = theme.typography.status + theme.spacing.sm;
    let (rect, _) = ui.allocate_exact_size(
        eframe::egui::vec2(width, height),
        eframe::egui::Sense::hover(),
    );

    ui.painter().text(
        rect.left_center(),
        eframe::egui::Align2::LEFT_CENTER,
        text,
        eframe::egui::FontId::proportional(theme.typography.status),
        theme.colors.text_muted.to_egui(),
    );
}
