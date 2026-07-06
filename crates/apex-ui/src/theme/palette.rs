#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApexColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl ApexColor {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn channels(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub(crate) fn to_egui(self) -> eframe::egui::Color32 {
        eframe::egui::Color32::from_rgba_premultiplied(self.r, self.g, self.b, self.a)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApexColors {
    pub app_background: ApexColor,
    pub surface: ApexColor,
    pub surface_raised: ApexColor,
    pub top_bar: ApexColor,
    pub status_bar: ApexColor,
    pub border: ApexColor,
    pub text: ApexColor,
    pub text_muted: ApexColor,
    pub accent: ApexColor,
    pub accent_hover: ApexColor,
    pub accent_pressed: ApexColor,
    pub accent_text: ApexColor,
    pub quiet_hover: ApexColor,
    pub quiet_pressed: ApexColor,
    pub disabled_fill: ApexColor,
    pub disabled_text: ApexColor,
    pub focus_ring: ApexColor,
    pub success: ApexColor,
}

impl Default for ApexColors {
    fn default() -> Self {
        Self {
            app_background: ApexColor::rgb(15, 18, 24),
            surface: ApexColor::rgb(24, 29, 38),
            surface_raised: ApexColor::rgb(31, 37, 48),
            top_bar: ApexColor::rgb(20, 24, 32),
            status_bar: ApexColor::rgb(18, 21, 28),
            border: ApexColor::rgb(57, 66, 82),
            text: ApexColor::rgb(232, 238, 246),
            text_muted: ApexColor::rgb(153, 166, 184),
            accent: ApexColor::rgb(105, 163, 255),
            accent_hover: ApexColor::rgb(129, 180, 255),
            accent_pressed: ApexColor::rgb(79, 136, 225),
            accent_text: ApexColor::rgb(8, 15, 28),
            quiet_hover: ApexColor::rgb(40, 48, 62),
            quiet_pressed: ApexColor::rgb(49, 59, 76),
            disabled_fill: ApexColor::rgb(42, 47, 56),
            disabled_text: ApexColor::rgb(104, 113, 128),
            focus_ring: ApexColor::rgb(167, 205, 255),
            success: ApexColor::rgb(99, 214, 159),
        }
    }
}
