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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())

        .init_resource::<CompositionWaves>()
        .init_resource::<PanelInfo>()
        .init_resource::<Scheduler>()

        .add_systems(Startup, (
            add, 
            setup_cameras,
            setup_schedules,
        ))

        // egui
        .add_systems(EguiPrimaryContextPass, (
            draw_panel,
            update_game_viewport,
        ).chain())
        

        .add_systems(Update, (
            draw_gizmos, 
            scheduler_add_frame,
        ))


        .run();
}
