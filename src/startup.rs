use bevy::camera::visibility::RenderLayers;
use bevy::ecs::schedule;

use crate::ecs_init::*;

use crate::internal_imports::*;

pub fn setup_cameras(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    // disable automatic context creation so that the ui camera can become egui primary context
    egui_global_settings.auto_create_primary_context = false;
    commands.spawn((
        Camera2d,
        MainCamera,
        Camera {
            order: 0,
            ..default()
        }
    ));
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        UICamera,
        PrimaryEguiContext,
        RenderLayers::none(),
    ));
}

pub fn add(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 0.5))),
        Transform::from_xyz(0.0, 600.0, 0.0)
    ));
}

pub fn setup_schedules(mut s: ResMut<Scheduler>) {
    s.add("ui_multiplier", 60);
}