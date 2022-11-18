# rosu_player
Beginner project, osu map parser and renderer (wip, does not work atm)
# running
just git clone it and `cargo run -- '<osu_file>' `
running on bash might have issues 
if your .osu file is in ``<osu_install_path>/osu!/``

due to the ! in the path,
current workaround is just to run `set +H`
before running the build

## Planned features
- .osu map player, view maps with video player like controls
- potentially(?) a replay viewer similar to osu!rewind