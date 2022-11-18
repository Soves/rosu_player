
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
}
#[derive(Debug, Default)]
pub struct Metadata {
    pub title: Option<String>,
}
#[derive(Debug, Default)]
pub struct Difficulty {
}
#[derive(Debug, Default)]
pub struct Events {
    pub backgrounds: Vec<events::Background>,
    pub videos: Vec<events::Video>,
    pub breaks: Vec<events::Break>,
}

#[derive(Debug, Default)]
pub struct TimingPoints {
}
#[derive(Debug, Default)]
pub struct Colours {
}
#[derive(Debug, Default)]
pub struct HitObjects {
}
#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i32),
}
