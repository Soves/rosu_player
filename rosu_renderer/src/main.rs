
use std::path::{Path, PathBuf};
use argh::FromArgs;

mod player;
use player::Player;

#[derive(FromArgs)]
/// play a beatmap
struct PlayBeatmap {

    /// path to .osu file
    #[argh(positional)]
    beatmap_path: Option<String>,

}

fn path_from_args(args: PlayBeatmap) -> Option<PathBuf> {
    if let Some(beatmap_path) = args.beatmap_path {
        return Some(PathBuf::from(beatmap_path));
    }
    None
}

fn main() {

    let beatmap_path = path_from_args(argh::from_env());

    Player::new(beatmap_path.unwrap()).run();

}