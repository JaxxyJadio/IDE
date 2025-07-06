use eframe::egui;

pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Add custom fonts here if needed
    
    ctx.set_fonts(fonts);
}

pub fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Dark theme configuration
    style.visuals = egui::Visuals::dark();
    
    // Customize spacing
    style.spacing.item_spacing = egui::Vec2::new(4.0, 4.0);
    style.spacing.button_padding = egui::Vec2::new(8.0, 4.0);
    
    // Customize colors for IDE-like appearance
    style.visuals.window_fill = egui::Color32::from_rgb(30, 30, 30);
    style.visuals.panel_fill = egui::Color32::from_rgb(37, 37, 38);
    style.visuals.faint_bg_color = egui::Color32::from_rgb(45, 45, 45);
    
    ctx.set_style(style);
}