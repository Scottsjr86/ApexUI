use crate::{response::ApexResponse, theme::ApexTheme};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ButtonKind {
    Primary,
    Quiet,
}

pub(crate) fn button(
    ui: &mut eframe::egui::Ui,
    theme: &ApexTheme,
    label: &str,
    kind: ButtonKind,
    enabled: bool,
) -> ApexResponse {
    let padding_x = theme.spacing.md;
    let width = (label.chars().count() as f32 * 7.75 + padding_x * 2.0).max(76.0);
    let height = match theme.density {
        crate::theme::ApexDensity::Compact => 28.0,
        crate::theme::ApexDensity::Comfortable => 32.0,
    };

    let sense = if enabled {
        eframe::egui::Sense::click()
    } else {
        eframe::egui::Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(eframe::egui::vec2(width, height), sense);

    let apex_response = ApexResponse::from_egui(&response, enabled);
    let fill = button_fill(theme, kind, &apex_response);
    let text = if enabled {
        match kind {
            ButtonKind::Primary => theme.colors.accent_text,
            ButtonKind::Quiet => theme.colors.text,
        }
    } else {
        theme.colors.disabled_text
    };

    ui.painter().rect(
        rect,
        eframe::egui::CornerRadius::same(theme.radius.md),
        fill.to_egui(),
        eframe::egui::Stroke::new(1.0, button_stroke(theme, kind, &apex_response).to_egui()),
        eframe::egui::StrokeKind::Inside,
    );

    if apex_response.focused() {
        ui.painter().rect_stroke(
            rect.expand(2.0),
            eframe::egui::CornerRadius::same(theme.radius.md + 2),
            eframe::egui::Stroke::new(1.5, theme.colors.focus_ring.to_egui()),
            eframe::egui::StrokeKind::Outside,
        );
    }

    ui.painter().text(
        rect.center(),
        eframe::egui::Align2::CENTER_CENTER,
        label,
        eframe::egui::FontId::proportional(theme.typography.button),
        text.to_egui(),
    );

    apex_response
}

fn button_fill(
    theme: &ApexTheme,
    kind: ButtonKind,
    response: &ApexResponse,
) -> crate::theme::ApexColor {
    if !response.enabled() {
        return theme.colors.disabled_fill;
    }

    match kind {
        ButtonKind::Primary if response.pressed() => theme.colors.accent_pressed,
        ButtonKind::Primary if response.hovered() => theme.colors.accent_hover,
        ButtonKind::Primary => theme.colors.accent,
        ButtonKind::Quiet if response.pressed() => theme.colors.quiet_pressed,
        ButtonKind::Quiet if response.hovered() => theme.colors.quiet_hover,
        ButtonKind::Quiet => theme.colors.surface_raised,
    }
}

fn button_stroke(
    theme: &ApexTheme,
    kind: ButtonKind,
    response: &ApexResponse,
) -> crate::theme::ApexColor {
    match (
        kind,
        response.enabled(),
        response.hovered() || response.focused(),
    ) {
        (_, false, _) => theme.colors.border,
        (ButtonKind::Primary, true, true) => theme.colors.focus_ring,
        (ButtonKind::Primary, true, false) => theme.colors.accent,
        (ButtonKind::Quiet, true, true) => theme.colors.focus_ring,
        (ButtonKind::Quiet, true, false) => theme.colors.border,
    }
}
