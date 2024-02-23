use bevy::prelude::*;
use bevy_water::WaterParam;

#[derive(Component, Default, Clone)]
pub struct WaterInteractable {
    pub water_line: f32,
    pub front: Vec3,
    pub back_left: Vec3,
    pub back_right: Vec3,
}

impl WaterInteractable {
    pub fn new(water_line: f32, front: f32, back: f32, left: f32, right: f32) -> Self {
        Self {
            water_line,
            front: Vec3::new(0.0, 0.0, front),
            back_left: Vec3::new(left, 0.0, back),
            back_right: Vec3::new(right, 0.0, back),
        }
    }

    pub fn sync_with_water(
        &self,
        water: &WaterParam,
        pos: Vec3,
        transform: &mut Transform,
        #[cfg(feature = "debug")] lines: &mut DebugLines,
    ) {
        let (yaw, _pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let global = Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(yaw));

        // Get the wave position at the front, back_left and back_right.
        let mut front = water.wave_point(global.transform_point(self.front));
        let left = water.wave_point(global.transform_point(self.back_left));
        let right = water.wave_point(global.transform_point(self.back_right));
        let normal = (left - front).cross(right - front).normalize();

        // Debug lines.
        #[cfg(feature = "debug")]
        {
            lines.line(front, front + normal, 0.0);
            lines.line_colored(front, right, 0.0, Color::RED);
            lines.line(right, left, 0.0);
            lines.line_colored(left, front, 0.0, Color::GREEN);
            lines.line(transform.translation, transform.translation + normal, 0.0);
        }

        front.y += self.water_line - 0.2;
        transform.look_at(front, normal);

        transform.translation.y = ((front.y + left.y + right.y) / 3.0) + self.water_line;
    }
}
