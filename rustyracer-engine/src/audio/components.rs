//! Audio components for Kira integration

use crate::prelude::*;
use crate::ecs::component::Component;
use crate::assets::handle::AssetHandle;

/// Audio clip asset data
#[derive(Clone, Debug)]
pub struct AudioClip {
    pub duration: f32,
    pub frame_count: usize,
}

impl Component for AudioClip {}

/// ECS component for audio source/emitter
#[derive(Clone, Debug)]
pub struct AudioSource {
    pub clip: AssetHandle<AudioClip>,
    pub volume: f32,
    pub pitch: f32,
    pub looping: bool,
    pub play_on_spawn: bool,
    pub range: f32,
    pub is_playing: bool,
    pub track_id: Option<kira::track::TrackId>,
}

impl Default for AudioSource {
    fn default() -> Self {
        Self {
            clip: AssetHandle::new(0),
            volume: 1.0,
            pitch: 1.0,
            looping: false,
            play_on_spawn: false,
            range: 10.0,
            is_playing: false,
            track_id: None,
        }
    }
}

impl Component for AudioSource {}

/// ECS component for audio listener
#[derive(Clone, Debug)]
pub struct AudioListener {
    pub active: bool,
}

impl Default for AudioListener {
    fn default() -> Self {
        Self { active: true }
    }
}

impl Component for AudioListener {}
