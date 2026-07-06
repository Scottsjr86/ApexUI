use super::{ApexColors, ApexDensity};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApexSpacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub window: f32,
}

impl Default for ApexSpacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            window: 18.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApexRadius {
    pub sm: u8,
    pub md: u8,
    pub lg: u8,
    pub pill: u8,
}

impl Default for ApexRadius {
    fn default() -> Self {
        Self {
            sm: 5,
            md: 8,
            lg: 12,
            pill: 64,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApexTypography {
    pub body: f32,
    pub button: f32,
    pub section: f32,
    pub status: f32,
}

impl Default for ApexTypography {
    fn default() -> Self {
        Self {
            body: 14.0,
            button: 14.0,
            section: 16.0,
            status: 12.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApexTheme {
    pub colors: ApexColors,
    pub spacing: ApexSpacing,
    pub radius: ApexRadius,
    pub typography: ApexTypography,
    pub density: ApexDensity,
}

impl ApexTheme {
    pub fn dark() -> Self {
        Self::default()
    }
}

impl Default for ApexTheme {
    fn default() -> Self {
        Self {
            colors: ApexColors::default(),
            spacing: ApexSpacing::default(),
            radius: ApexRadius::default(),
            typography: ApexTypography::default(),
            density: ApexDensity::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ApexTheme;

    #[test]
    fn default_theme_has_apex_owned_tokens() {
        let theme = ApexTheme::default();

        assert!(theme.spacing.md > theme.spacing.sm);
        assert!(theme.radius.lg > theme.radius.sm);
        assert_eq!(theme.colors.accent.channels().3, 255);
    }
}
