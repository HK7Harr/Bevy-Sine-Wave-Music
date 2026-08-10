mod internal_imports;
use internal_imports::*;

mod startup;
use startup::*;

mod update;
use update::*;

mod ecs_init;
use ecs_init::*;

mod helpers;

pub const DEFAULT_PANEL_WIDTH: f32 = 400.0;
pub const DEFAULT_BODY_FONT_SIZE: f32 = 32.0;
pub const DEFAULT_HEADER_FONT_SIZE: f32 = 46.0;

pub const PIXELS_TO_MUSIC_RATIO: f64 = 160.0; // 160 pixels to 1 music unit
pub const PIXELS_TO_SECONDS_RATIO: f64 = 100.0; // 100 pixeels (x) is one second of platytime

pub const PIXEL_PER_SEGMENT: f64 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())

        .init_resource::<CompositionWaves>()
        .init_resource::<PanelInfo>()
        .init_resource::<Scheduler>()

        .add_systems(Startup, ( 
            setup_cameras,
            setup_schedules,
        ))

        // egui
        .add_systems(EguiPrimaryContextPass, (
            draw_panel,
        ))
        

        .add_systems(Update, (
            draw_collective_music_gizmos, 
            draw_composing_music_gizmos,
            scheduler_add_frame,
        ))


        .run();
}
