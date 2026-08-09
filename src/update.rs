use crate::internal_imports::*;



pub fn draw_panel(
    mut contexts: EguiContexts,    
    mut info: ResMut<PanelInfo>,
    scheduler: Res<Scheduler>,
    window: Single<&Window, With<PrimaryWindow>>,    
) {
    if window.physical_width() == 0 || window.physical_height() == 0 {
        return;
    }

    let ctx = contexts.ctx_mut().unwrap();

    let mut viewport_ui = egui::Ui::new(
        ctx.clone(),
        "viewport".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    if scheduler.check("ui_multiplier") {
        info.update_ui_scalar();
    }
    viewport_ui.style_mut().text_styles.insert(
        egui::TextStyle::Body, 
        egui::FontId::new(DEFAULT_BODY_FONT_SIZE * info.ui_scalar, egui::FontFamily::Proportional)
    );

    viewport_ui.style_mut().text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(DEFAULT_HEADER_FONT_SIZE * info.ui_scalar, egui::FontFamily::Proportional),
    );


    // UI 
    let left_logical_width = egui::panel::Panel::left("left_panel") // Note: Standard egui uses SidePanel
        .resizable(true)
        .size_range(DEFAULT_PANEL_WIDTH..=700.0)
        .show(&mut viewport_ui, |ui| {
            ui.heading("Sim Info");
            ui.label("label1");

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width(); 

    info.width = left_logical_width;
}


pub fn update_game_viewport(
    window: Single<&Window, With<PrimaryWindow>>,      
    mut camera_query: Query<&mut Camera, With<MainCamera>>,   
    info: Res<PanelInfo>
) {
    // converting from egui logical units to viewport 
    let scale_factor = window.scale_factor();
    let left_physical_width = (info.width * scale_factor) as u32;

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



pub fn draw_gizmos(
    mut gizmos: Gizmos,
    waves: Res<CompositionWaves>
) {
    let downward_length: f32 = waves.waves.len() as f32 * 400.0;

    // marglinje
    gizmos.line_2d(
        Vec2 { x: 100.0, y: 720.0 },
        Vec2 { x: 100.0, y: 720.0 - downward_length },
        CRIMSON
    );
}

pub fn scheduler_add_frame(mut scheduler: ResMut<Scheduler>) {
    scheduler.add_frame();
}