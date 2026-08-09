use crate::internal_imports::*;

pub fn query_viewport_center(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Vec2 {
    let Ok((camera, camera_transform)) = camera_query.single() else { panic!("query_viewport_center: Error1")};

    if let Some(viewport_rect) = camera.logical_viewport_rect() {

        let viewport_center_screen = viewport_rect.center();

        // converts to world units (x,y)
        if let Ok(world_center) = camera.viewport_to_world_2d(camera_transform, viewport_center_screen) {
            return world_center;
        }
    }
    panic!("query_viewport_center: Error1");
}