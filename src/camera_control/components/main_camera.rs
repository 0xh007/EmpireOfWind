use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The `MainCamera` component is used to mark the primary camera entity in the game.
/// This component allows systems to identify and manipulate the main camera specifically.
///
/// # Usage
///
/// The `MainCamera` component is attached to the primary camera entity during setup.
/// This primary camera can have various settings, including the camera's position,
/// projection type, render layers, fog settings, and custom camera behaviors like zoom.
///
/// The `MainCamera` component is integral to identifying and controlling the primary camera in the game,
/// ensuring that it interacts correctly with various systems and player inputs.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct MainCamera;
