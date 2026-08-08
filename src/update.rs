use crate::internal_imports::*;


// Dummy struct based on your snippet
#[derive(Resource)]
pub struct PanelInfo {
    pub is_paused: bool,
    pub time_multiplier: f32,
}

pub fn draw_panel(
    mut contexts: EguiContexts, 
    window: Single<&Window, With<PrimaryWindow>>,      
    mut camera_query: Query<&mut Camera, With<MainCamera>>,         
) {
    let ctx = contexts.ctx_mut().unwrap(); 

    let mut viewport_ui = egui::Ui::new(
        ctx.clone(),
        "viewport".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    viewport_ui.style_mut().text_styles.insert(
        egui::TextStyle::Body, 
        egui::FontId::new(BODY_FONT_SIZE, egui::FontFamily::Proportional)
    );

    viewport_ui.style_mut().text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(HEADER_FONT_SIZE, egui::FontFamily::Proportional),
    );


    // UI 
    let left_logical_width = egui::panel::Panel::left("left_panel") // Note: Standard egui uses SidePanel
        .resizable(false)
        .min_size(400.0)
        .show(&mut viewport_ui, |ui| {
            ui.heading("Sim Info");

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width(); 
    // converting from egui logical units to viewport 
    let scale_factor = window.scale_factor();
    let left_physical_width = (left_logical_width * scale_factor) as u32;

    let window_physical_width = window.physical_width();
    let window_physical_height = window.physical_height();

    // safley update the cameras ciewport
    if let Ok(mut camera) = camera_query.single_mut() {
        // prevent panics if the window is minimized to much
        if window_physical_width > left_physical_width && window_physical_height > 0 {
            camera.viewport = Some(Viewport {
                // Push the render to the right side of the panel
                physical_position: UVec2::new(left_physical_width, 0),
                // dill the rest of the screen
                physical_size: UVec2::new(
                    window_physical_width.saturating_sub(left_physical_width),
                    window_physical_height,
                ),
                ..default()
            });
        } else {
            // if the screen is too small to split, drop the viewport override
            camera.viewport = None;
        }
    }
    
}