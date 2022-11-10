use std::path::{Path, PathBuf};
use glob::glob;

/// finds the beatmapset with the matching name
/// on the osu! install directory
/// 
/// # returns
/// a collection of beatmap paths

pub fn find_beatmap_set(osu_path: &Path, beatmap_set: String)
    -> Result<Vec<PathBuf>, Error>{

    let path_str = osu_path.display();
    
    let beatmap_set_pattern = format!(
        "{}/Songs/*{}*", 
        path_str,
        beatmap_set
    );

    let mut beatmap_sets = glob(&beatmap_set_pattern)
        .map_err(|_| Error::SetNotFound(
            format!("beamap set {} not found", beatmap_set)
            .to_string()
        ))?;

    let first_match = beatmap_sets.next()
        .ok_or(Error::SetNotFound(
            format!("beamap set {} not found", beatmap_set)
            .to_string()
        ))?
        .map_err(|_| Error::SetNotFound(
            "first beatmap set found is invalid"
            .to_string()
        ))?;

    let beatmap_pattern = format!(
        "{}/*.osu", 
        first_match.display()
    );
    println!("{}",beatmap_pattern);
    let beatmap_pattern = beatmap_pattern.as_str();

    let beatmaps = glob(beatmap_pattern)
        .map_err(|_| Error::MapNotFound(
            "no beatmaps in the se were found"
            .to_string()
        ))?;

    Ok(beatmaps.flatten().collect())
}

#[derive(Debug)]
pub enum Error {
    SetNotFound(String),
    MapNotFound(String)
}