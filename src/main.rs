mod internal_imports;
use internal_imports::*;

mod startup;
use startup::*;

mod update;
use update::*;

mod ecs_init;
use ecs_init::*;

pub const BODY_FONT_SIZE: f32 = 20.0;
pub const HEADER_FONT_SIZE: f32 = 32.0;
pub const SLIDER_WIDTH: f32 = 400.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())

        .add_systems(Startup, (add, setup_cameras))
        .add_systems(EguiPrimaryContextPass, draw_panel)
        .run();
}
