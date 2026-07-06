use crate::{theme::ApexTheme, ui::ApexUi};

pub(crate) fn show_panel(
    ui: &mut eframe::egui::Ui,
    theme: &ApexTheme,
    title: String,
    body: impl FnOnce(&mut ApexUi<'_>),
) {
    let frame = eframe::egui::Frame::new()
        .fill(theme.colors.surface.to_egui())
        .stroke(eframe::egui::Stroke::new(
            1.0,
            theme.colors.border.to_egui(),
        ))
        .corner_radius(eframe::egui::CornerRadius::same(theme.radius.lg))
        .inner_margin(eframe::egui::Margin::same(theme.spacing.lg as i8));

    frame.show(ui, |ui| {
        section_header(ui, theme, &title);
        ui.add_space(theme.spacing.md);
        let mut apex_ui = ApexUi::new(ui, theme);
        body(&mut apex_ui);
    });
}

pub(crate) fn show_card(
    ui: &mut eframe::egui::Ui,
    theme: &ApexTheme,
    body: impl FnOnce(&mut ApexUi<'_>),
) {
    let frame = eframe::egui::Frame::new()
        .fill(theme.colors.surface_raised.to_egui())
        .stroke(eframe::egui::Stroke::new(
            1.0,
            theme.colors.border.to_egui(),
        ))
        .corner_radius(eframe::egui::CornerRadius::same(theme.radius.md))
        .inner_margin(eframe::egui::Margin::same(theme.spacing.md as i8));

    frame.show(ui, |ui| {
        let mut apex_ui = ApexUi::new(ui, theme);
        body(&mut apex_ui);
    });
}

pub(crate) fn section_header(ui: &mut eframe::egui::Ui, theme: &ApexTheme, text: &str) {
    let height = theme.typography.section + theme.spacing.sm;
    let width = ui.available_width().max(160.0);
    let (rect, _) = ui.allocate_exact_size(
        eframe::egui::vec2(width, height),
        eframe::egui::Sense::hover(),
    );

    ui.painter().text(
        rect.left_center(),
        eframe::egui::Align2::LEFT_CENTER,
        text,
        eframe::egui::FontId::proportional(theme.typography.section),
        theme.colors.text.to_egui(),
    );
}

pub(crate) fn body_text(ui: &mut eframe::egui::Ui, theme: &ApexTheme, text: &str) {
    let height = theme.typography.body + theme.spacing.sm;
    let width = ui.available_width().max(160.0);
    let (rect, _) = ui.allocate_exact_size(
        eframe::egui::vec2(width, height),
        eframe::egui::Sense::hover(),
    );

    ui.painter().text(
        rect.left_center(),
        eframe::egui::Align2::LEFT_CENTER,
        text,
        eframe::egui::FontId::proportional(theme.typography.body),
        theme.colors.text_muted.to_egui(),
    );
}
