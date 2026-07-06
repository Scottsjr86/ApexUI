use std::{error::Error, fmt};

use crate::{egui_bridge::runtime::EframeApexAdapter, frame::ApexFrame};

pub trait ApexApp: 'static {
    fn update(&mut self, frame: &mut ApexFrame<'_>);
}

pub type ApexRunResult = Result<(), ApexRunError>;

#[derive(Debug)]
pub struct ApexRunError {
    source: eframe::Error,
}

impl fmt::Display for ApexRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Apex runtime failed: {}", self.source)
    }
}

impl Error for ApexRunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl From<eframe::Error> for ApexRunError {
    fn from(source: eframe::Error) -> Self {
        Self { source }
    }
}

pub fn run_native<A: ApexApp>(title: impl Into<String>, app: A) -> ApexRunResult {
    let title = title.into();
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([900.0, 620.0])
            .with_min_inner_size([640.0, 420.0]),
        ..Default::default()
    };

    eframe::run_native(
        &title,
        native_options,
        Box::new(move |cc| Ok(Box::new(EframeApexAdapter::new(app, cc)))),
    )
    .map_err(ApexRunError::from)
}
