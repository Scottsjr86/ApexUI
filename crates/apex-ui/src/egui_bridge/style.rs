use crate::theme::ApexTheme;

pub(crate) fn apply_theme_to_context(ctx: &eframe::egui::Context, theme: &ApexTheme) {
    let mut visuals = eframe::egui::Visuals::dark();
    visuals.window_fill = theme.colors.app_background.to_egui();
    visuals.panel_fill = theme.colors.app_background.to_egui();
    visuals.widgets.noninteractive.bg_fill = theme.colors.surface.to_egui();
    visuals.widgets.noninteractive.fg_stroke.color = theme.colors.text.to_egui();
    visuals.widgets.inactive.bg_fill = theme.colors.surface_raised.to_egui();
    visuals.widgets.inactive.fg_stroke.color = theme.colors.text.to_egui();
    visuals.widgets.hovered.bg_fill = theme.colors.quiet_hover.to_egui();
    visuals.widgets.hovered.fg_stroke.color = theme.colors.text.to_egui();
    visuals.widgets.active.bg_fill = theme.colors.quiet_pressed.to_egui();
    visuals.widgets.active.fg_stroke.color = theme.colors.text.to_egui();
    visuals.selection.bg_fill = theme.colors.accent.to_egui();
    visuals.selection.stroke.color = theme.colors.accent_text.to_egui();

    ctx.set_visuals(visuals);
}
