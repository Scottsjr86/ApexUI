use crate::{
    response::{ApexInteractionState, ApexResponse},
    theme::ApexTheme,
};

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
    match (kind, response.interaction_state()) {
        (_, ApexInteractionState::Disabled) => theme.colors.disabled_fill,
        (ButtonKind::Primary, ApexInteractionState::Pressed) => theme.colors.accent_pressed,
        (ButtonKind::Primary, ApexInteractionState::Hovered) => theme.colors.accent_hover,
        (ButtonKind::Primary, _) => theme.colors.accent,
        (ButtonKind::Quiet, ApexInteractionState::Pressed) => theme.colors.quiet_pressed,
        (ButtonKind::Quiet, ApexInteractionState::Hovered) => theme.colors.quiet_hover,
        (ButtonKind::Quiet, _) => theme.colors.surface_raised,
    }
}

fn button_stroke(
    theme: &ApexTheme,
    kind: ButtonKind,
    response: &ApexResponse,
) -> crate::theme::ApexColor {
    match (kind, response.interaction_state()) {
        (_, ApexInteractionState::Disabled) => theme.colors.border,
        (ButtonKind::Primary, ApexInteractionState::Hovered | ApexInteractionState::Focused) => {
            theme.colors.focus_ring
        }
        (ButtonKind::Primary, _) => theme.colors.accent,
        (ButtonKind::Quiet, ApexInteractionState::Hovered | ApexInteractionState::Focused) => {
            theme.colors.focus_ring
        }
        (ButtonKind::Quiet, _) => theme.colors.border,
    }
}

#[cfg(test)]
mod tests {
    use crate::{response::ApexResponse, theme::ApexTheme};

    use super::{ButtonKind, button_fill, button_stroke};

    #[test]
    fn button_fill_uses_apex_interaction_state_order() {
        let theme = ApexTheme::default();
        let disabled = ApexResponse::new(true, true, true, true, false);
        let pressed = ApexResponse::new(false, true, true, true, true);
        let hovered = ApexResponse::new(false, true, false, true, true);
        let idle = ApexResponse::new(false, false, false, false, true);

        assert_eq!(
            button_fill(&theme, ButtonKind::Primary, &disabled),
            theme.colors.disabled_fill
        );
        assert_eq!(
            button_fill(&theme, ButtonKind::Primary, &pressed),
            theme.colors.accent_pressed
        );
        assert_eq!(
            button_fill(&theme, ButtonKind::Primary, &hovered),
            theme.colors.accent_hover
        );
        assert_eq!(
            button_fill(&theme, ButtonKind::Primary, &idle),
            theme.colors.accent
        );
    }

    #[test]
    fn button_stroke_treats_focus_and_hover_as_visible_attention() {
        let theme = ApexTheme::default();
        let focused = ApexResponse::new(false, false, false, true, true);
        let hovered = ApexResponse::new(false, true, false, false, true);
        let disabled = ApexResponse::new(false, true, false, true, false);

        assert_eq!(
            button_stroke(&theme, ButtonKind::Quiet, &focused),
            theme.colors.focus_ring
        );
        assert_eq!(
            button_stroke(&theme, ButtonKind::Quiet, &hovered),
            theme.colors.focus_ring
        );
        assert_eq!(
            button_stroke(&theme, ButtonKind::Quiet, &disabled),
            theme.colors.border
        );
    }
}
