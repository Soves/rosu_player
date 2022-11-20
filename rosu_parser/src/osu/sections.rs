
//TODO: fill rest of the fields for all sections

use std::path::PathBuf;
use super::events::{self};

#[derive(Debug, Default)]
pub struct General {
    pub audio_filename: Option<PathBuf>,
    pub audio_lead_in: Option<isize>,
    pub audio_hash: Option<String>, //deprecated
    pub preview_time: Option<isize>,
    pub countdown: Option<usize>,
    pub sample_set: Option<String>,
    pub stack_leniency: Option<f32>,
    pub mode: Option<usize>,
    pub letter_box_in_breaks: Option<bool>,
    pub story_fire_in_front: Option<bool>,  //deprecated
    pub use_skin_sprites: Option<bool>,
    pub always_show_playfield: Option<bool>,  //deprecated
    pub overlay_position: Option<String>,
    pub skin_preference: Option<String>,
    pub epilepsy_warning: Option<bool>,
    pub countdown_offset: Option<isize>,
    pub special_style: Option<bool>,
    pub widescreen_storyboard: Option<bool>,
    pub samples_match_playback_rate: Option<bool>,
}
#[derive(Debug, Default)]
pub struct Editor {
    pub bookmarks: Option<String>, //comma separated list of ints
    pub distance_spacing: Option<f32>,
    pub beat_divisor: Option<usize>,
    pub grid_size: Option<usize>,
    pub timeline_zoom: Option<f32>,
}
#[derive(Debug, Default)]
pub struct Metadata {
    pub title: Option<String>,
    pub title_unicode: Option<String>,
    pub artist: Option<String>,
    pub artist_unicode: Option<String>,
    pub creator: Option<String>,
    pub version: Option<String>,
    pub source: Option<String>,
    pub tags: Option<String>, //TODO: split list
    pub beatmap_id: Option<usize>,
    pub beatmap_set_id: Option<usize>,
}
#[derive(Debug, Default)]
pub struct Difficulty {
    pub hp_drain_rate: Option<f32>,
    pub circle_size: Option<f32>,
    pub overall_difficulty: Option<f32>,
    pub approach_rate: Option<f32>,
    pub slider_multiplier: Option<f32>,
    pub slider_tick_rate: Option<f32>,
}
#[derive(Debug, Default)]
pub struct Events {
    pub backgrounds: Vec<events::Background>,
    pub videos: Vec<events::Video>,
    pub breaks: Vec<events::Break>,
}

pub type TimingPoints = Vec<TimingPoint>;

#[derive(Debug, Default)]
pub struct TimingPoint {
    pub time: usize,
    pub beat_length: f32,
    pub meter: usize,
    pub sample_set: usize,
    pub sample_index: usize,
    pub volume: usize,
    pub uninherited: bool,
    pub effects: usize,
}

pub type Colours = Vec<Colour>;

#[derive(Debug, Default)]
pub struct Colour {
    pub combo: u8,
    pub slider_track_override: u8,
    pub slider_border: u8,
}

pub type HitObjects = Vec<HitObject>;

#[derive(Debug, Default)]
pub struct HitObject {
    pub x: usize,
    pub y: usize,
    pub time: usize,
    pub r#type: usize,
    pub hit_sound: usize,
    pub object_params: Option<String>, //TODO: split comma list
    pub hit_sample: Option<String>, // TODO: split colon list
}


#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i32),
}
