use crate::internal_imports::*;



pub fn draw_panel(
    mut contexts: EguiContexts,    
    mut info: ResMut<PanelInfo>,
    scheduler: Res<Scheduler>,
    window: Single<&Window, With<PrimaryWindow>>,  
    mut camera_query: Query<&mut Camera, With<MainCamera>>,   
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
pub fn draw_collective_music_gizmos(
    mut gizmos: Gizmos,
    waves: Res<CompositionWaves>,
    camera_query: Query<(&Transform, &Projection), With<MainCamera>>,
) {
    let Ok((cam_transform, projection)) = camera_query.single() else { return; };

    // Extract the inner OrthographicProjection struct from the Projection enum component
    if let Projection::Orthographic(ortho) = projection {
        // Calculate the top y of the viewport in world space
        let viewport_top_y = cam_transform.translation.y + (ortho.area.height() / 2.0);
        let header_y = viewport_top_y - 360.0;

        // draw red header
        gizmos.line_2d(
            Vec2::new(-1280.0, header_y),
            Vec2::new(15000.0, header_y),
            RED,
        );
    }
}   

pub fn draw_composing_music_gizmos(
    mut gizmos: Gizmos,
    waves: Res<CompositionWaves>
) {
    let separators: u32 = waves.waves.len() as u32 + 1;

    // separator linws
    for i in 1..=separators {
        match i {
            1 => {}
            2 => {
                gizmos.line_2d(
                Vec2 { x: -1280.0, y:  0.0}, 
                Vec2 { x: 15000.0, y: 0.0 }, 
                DARK_GREEN
                );
            }
            _ => {
                gizmos.line_2d(
                Vec2 { x: -1280.0, y: (i as f32 - 2.0)* -360.0}, 
                Vec2 { x: 15000.0, y: (i as f32 - 2.0)* -360.0}, 
                DARK_GREEN
                );
            }
        }
    }

}

pub fn draw_sine_gizmo(gizmos: &mut Gizmos, sine: &SineWave, index: u32) {
    let segments = (sine.x_stop - sine.x_start / PIXEL_PER_SEGMENT).floor() as i32;
    let total_time = sine.x_stop - sine.x_start / PIXELS_TO_SECONDS_RATIO;
    let total_music_units = sine.x_stop - sine.x_start / PIXELS_TO_MUSIC_RATIO;
    
    let time_units_segment_size = total_time / segments as f64;
    let music_units_segment_size = total_music_units / segments as f64;

    let mut points_with_color: Vec<(Vec2, Srgba)> = Vec::new();

    for i in 1..segments {
        let xmu = i as f64 * music_units_segment_size;
        let y = ((xmu * sine.phase_offset).sin() * sine.amplitude * 160.0 + 180.0) - (index * 360) as f64;
        let x = i as f64 * PIXEL_PER_SEGMENT;

        let position = Vec2 {x: x as f32, y: y as f32};
        let color: Srgba;
        if i as f64 * time_units_segment_size <= sine.attack {color = RED}
        else if i as f64 * time_units_segment_size <= sine.attack + sine.decay {}
    }
}

pub fn scheduler_add_frame(mut scheduler: ResMut<Scheduler>) {
    scheduler.add_frame();
}