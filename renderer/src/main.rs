mod game;
use std::path::Path;

use game::Game;
use std::env;

fn main() {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);//remove path it was run from the args
    //consider using this later by cd-ing into osu directory
    //so it doesnt need to be passed as a arg
    //cargo run wont really work with that though
    
    if args.len() == 2 {
        let osu_directory = Path::new(&args[0]);

        //TODO: add in difficulty name, map id and better overall cli
        let beatmaps = osu_parser::file::find_beatmap_set(
            osu_directory, args[1].clone()).unwrap();

        println!("file is here: {:?}", beatmaps[0]);

    } else {
        println!("osu_parser <osu_directory> <beatmap name>");
    }

    //TODO: dont block the main thread so we can still interact with the cli
    Game::run();
}