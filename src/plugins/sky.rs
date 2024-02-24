use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_atmosphere::prelude::*;

use bevy::time::Stopwatch;
use bevy_water::{ImageReformat, ImageUtilsPlugin};

const SPEED_MIN: f32 = 0.05;
const SPEED_DELTA: f32 = 0.01;
const SPEED_MAX: f32 = 1.0;

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4 * 1024 })
            .insert_resource(AtmosphereModel::new(Nishita {
                sun_position: Vec3::new(0.0, 1.0, 1.0),
                ..default()
            }))
            .add_plugins(ImageUtilsPlugin)
            .add_plugins(AtmospherePlugin)
            .insert_resource(CycleTimer::new(Duration::from_millis(1000), 0.2))
            .add_systems(Startup, setup_atmosphere)
            .add_systems(Update, timer_control)
            .add_systems(Update, daylight_cycle);
    }
}

#[derive(Component)]
struct Sun;

// Timer for updating the daylight cycle (updating the atmosphere every frame is slow, so it's better to do incremental changes)
#[derive(Resource)]
struct CycleTimer {
    update: Timer,
    time: Stopwatch,
    speed: f32,
}

impl CycleTimer {
    pub fn new(duration: Duration, speed: f32) -> Self {
        Self {
            update: Timer::new(duration, TimerMode::Repeating),
            time: Stopwatch::new(),
            speed,
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        if !self.paused() {
            self.update.tick(delta);
            self.time.tick(delta.mul_f32(self.speed));
        }
    }

    pub fn paused(&self) -> bool {
        self.time.paused()
    }

    pub fn toggle_pause(&mut self) {
        if self.time.paused() {
            self.time.unpause();
        } else {
            self.time.pause();
        }
    }

    pub fn time(&self) -> f32 {
        self.time.elapsed().as_millis() as f32 / 2000.0
    }

    pub fn update(&self) -> bool {
        self.update.finished()
    }

    pub fn update_speed(&mut self, delta: f32) {
        self.speed += delta;
        if self.speed < SPEED_MIN {
            self.speed = SPEED_MIN;
        }
        if self.speed > SPEED_MAX {
            self.speed = SPEED_MAX;
        }
    }
}

fn timer_control(input: Res<Input<KeyCode>>, mut timer: ResMut<CycleTimer>) {
    if input.just_pressed(KeyCode::P) {
        timer.toggle_pause();
    }
    if input.pressed(KeyCode::NumpadAdd) {
        timer.update_speed(SPEED_DELTA);
        eprintln!("Increase speed: {}", timer.speed);
    }
    if input.pressed(KeyCode::NumpadSubtract) {
        timer.update_speed(-SPEED_DELTA);
        eprintln!("Decrease speed: {}", timer.speed);
    }
}

// We can edit the Atmosphere resource and it will be updated automatically
fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    // Do nothing if timer is paused.
    if timer.paused() {
        return;
    }

    timer.tick(time.delta());

    if timer.update() {
        let mut pos = atmosphere.sun_position;
        let t = (timer.time() + 3.0) * 0.1;
        pos.y = t.sin();
        pos.z = t.cos();
        atmosphere.sun_position = pos;

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-pos.y.atan2(pos.z));
            directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
        }
    }
}

fn setup_atmosphere(
    // TODO: Use bevy_asset_loader
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // "Sun"
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 11127.65,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_x(-0.340)),
            ..default()
        })
        .insert(Sun); // Marks the light as Sun
}
