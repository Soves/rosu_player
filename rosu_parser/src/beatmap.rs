use std::{str::{Bytes}, path::PathBuf, fs};

pub mod events;
pub mod sections;
use sections::*;

#[derive(Debug)]
pub struct Beatmap {
    pub version: Option<usize>,

    pub general: Option<General>,
    pub editor: Option<Editor>,
    pub metadata: Option<Metadata>,
    pub difficulty: Option<Difficulty>,
    pub events: Option<Events>,
    pub timing_points: Option<TimingPoints>,
    pub colours: Option<Colours>,
    pub hit_objects: Option<HitObjects>,
}

impl Beatmap {
    
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load_from_string(string: String) -> Result<Beatmap, Error> {
        let mut parser = Parser::new(string.bytes());
        parser.parse()
    }

    pub fn load_from_file(filename: &PathBuf) -> Result<Beatmap, Error> {
        Beatmap::load_from_string(fs::read_to_string(filename).unwrap())
    }

}

impl Default for Beatmap {
    fn default() -> Self {
        Self {
            version: None,
            general: Default::default(),
            editor: Default::default(),
            metadata: Default::default(),
            difficulty: Default::default(),
            events: Default::default(),
            timing_points: Default::default(),
            colours: Default::default(),
            hit_objects: Default::default()
        }
    }
}

enum Section {
    General(General),
    Editor(Editor),
    Metadata(Metadata),
    Difficulty(Difficulty),
    Events(Events),
    TimingPoints(TimingPoints),
    Colours(Colours),
    HitObjects(HitObjects),
}

struct Parser<'a> {
    chr: Option<u8>,          //curent char
    section: Option<Section>, //current section
    key: Option<String>,      //current key
    reader: Bytes<'a>,        //char iterator
    line: usize,              //current line
    col: usize,               //currennt column
    result: Option<Beatmap>,
}

impl<'a> Parser<'a> {

    fn new(reader: Bytes<'a>) -> Self {
        let mut parser = Self {
            chr: None,
            section: None,
            key: None,
            reader,
            line: 0,
            col: 0,
            result: None
        };

        parser.bump();
        parser
    }

    /// checks if we are at the end of the file
    fn eof(&self) -> bool{
        self.chr.is_none()
    }

    /// shifts current char we are operating on to the next one
    fn bump(&mut self) {
        //get next character from iterator
        self.chr = self.reader.next();

        match self.chr {
            Some(b'\n') => {
                self.line += 1;
                self.col = 0;
            },
            Some(..) => {
                self.col += 1;
            },
            None => {}
        }
    }

    fn error<U, M: Into<String>>(&self, msg: M) -> Result<U, Error> {
        Err(Error::Parse(msg.into()))
    }

    fn parse_whitespace(&mut self) {
        
        while let Some(c) = self.chr {
            if !c.is_ascii_whitespace() && 
                c != b'\n' && 
                c != b'\t' && 
                c != b'\r' {
                break;
            }
            self.bump();
        }
    }

    fn parse_whitespace_except_line_break(&mut self) {
        while let Some(c) = self.chr {
            if (c != b'\n' && 
                c != b'\r' && 
                !c.is_ascii_whitespace()) &&
                c != b'\t' {
                break;
            }
            self.bump();
        }
    }

    fn parse(&mut self) -> Result<Beatmap, Error> {
        
        self.result = Some(Beatmap::new());
        self.key = None;

        self.parse_whitespace();

        while let Some(cur_chr) = self.chr {
            match cur_chr {
                b'/' | b'#' => {
                    if self.col > 1 {
                        return self.error("doesn't supprt inline comments");
                    }

                    //not necessary, single slash comments would be fine, just to be closer to spec
                    if cur_chr == b'/'{
                        if let Some(next_char) = self.reader.by_ref().peekable().next(){
                            if next_char != b'/'{
                                return self.error("only one \"/\" found, expecting 2");
                            }
                        }
                    }

                    self.parse_comment();
                }
                b'[' => self.parse_section()?,
                b'=' | b':' => {
                    if let None = self.key {
                        return self.error("missing key");
                    }

                    self.parse_val()?
                }
                _ => self.parse_property()?,
            }

            self.parse_whitespace();
        }

        //make sure the last section gets written to result
        if let Some(sec) = self.section.take() {
            
            self.finish_section(sec)?;
        }

        if let Some(res) = self.result.take() {
            return Ok(res)
        }

        self.error("No result")
    }

    fn parse_comment(&mut self) {
        while let Some(c) = self.chr {
            self.bump();
            if c == b'\n' {
                break;
            }
        }
    }

    fn parse_str_until(&mut self, endpoint: &[Option<u8>]) -> Result<String, Error> {

        let mut result = String::new();

        while !endpoint.contains(&self.chr) {
            match self.chr {
                None => {
                    return self.error(
                        format!(
                            "expecting \"{:?}\" but found EOF", endpoint
                        ));
                }
                Some(c) => {
                    result.push(c.into());
                }
            }
            self.bump();
        }

        Ok(result)
    }

    fn parse_section(&mut self) -> Result<(), Error>{

        self.bump();
        let section_str = self.parse_str_until(&[Some(b']')])?;
        let section_str = section_str.trim();
        self.bump();
        
        let next_section;
        next_section = match section_str {
        
            "General" => Section::General(General::default()),
            "Editor" => Section::Editor(Editor::default()),
            "Metadata" => Section::Metadata(Metadata::default()),
            "Difficulty" => Section::Difficulty(Difficulty::default()),
            "Events" => Section::Events(Events::default()),
            "TimingPoints" => Section::TimingPoints(TimingPoints::default()),
            "Colours" => Section::Colours(Colours::default()),
            "HitObjects" => Section::HitObjects(HitObjects::default()),

            _ => {
                //undefined section
                self.section = None; //reset section so we dont write to previous section
                return self.error(format!(
                    "Undefined section \"{}\"", 
                    section_str
                ));
            }
        };
        
        if let Some(sec) = self.section.take() {
            self.finish_section(sec)?;
        }
        
        self.section = Some(next_section);
        Ok(())
    }

    fn parse_property(&mut self) -> Result<(), Error> {

        match self.section {
            
            Some(Section::General(_)) => self.parse_key(),
            Some(Section::Editor(_)) => self.parse_key(),
            Some(Section::Metadata(_)) => self.parse_key(),
            Some(Section::Difficulty(_)) => self.parse_key(),
            Some(Section::Events(_)) => self.parse_list(),
            Some(Section::TimingPoints(_)) => self.parse_list(),
            Some(Section::Colours(_)) => self.parse_key(),
            Some(Section::HitObjects(_)) => self.parse_list(),
            None => {
                //we dont mind not having a section, just ignore preperties until next section
                //we match for the top level version number here
                if self.line == 0{
                    let version_str = self.parse_str_until_eol()?;

                    if let Some(res) = self.result.as_mut() {
                        if let Some(version_num) = version_str
                            .split(" ").last() {

                            res.version = Some(
                                version_num
                                    .trim_start_matches("v")
                                    .parse()
                                    .unwrap()
                            );
                        }
                    }

                }

                return Ok(());
            }

        }

    }

    fn parse_list(&mut self) -> Result<(), Error> {
        let list = self.parse_str_until_eol()?;

        let mut params = list.split(",");

        //TODO: handle gracefully and clean it up
        match self.section.as_mut() {
            Some(Section::Events(e)) => match params.next() { //event type
                Some("0") => e.backgrounds.push(events::Background{ 
                    start_time: params.next().unwrap().parse().unwrap(),
                    filename: PathBuf::from(params.next().unwrap().trim_matches('"')),
                    x_offset: params.next().unwrap().parse().unwrap(),
                    y_offset: params.next().unwrap().parse().unwrap(),
                }),
                Some("1") | Some("Video") => e.videos.push(events::Video{ 
                    start_time: params.next().unwrap().parse().unwrap(),
                    filename: PathBuf::from(params.next().unwrap()),
                    x_offset: params.next().unwrap().parse().unwrap(),
                    y_offset: params.next().unwrap().parse().unwrap(),
                }),
                Some("2") | Some("Break") => e.breaks.push(events::Break{ 
                    start_time: params.next().unwrap().parse().unwrap(),
                    end_time: params.next().unwrap().parse().unwrap(),
                }),
                _ => {}
            },
            Some(Section::TimingPoints(s)) => {
                s.push(TimingPoint { 
                    time: params.next().unwrap().parse().unwrap(),
                    beat_length: params.next().unwrap().parse().unwrap(),
                    meter: params.next().unwrap().parse().unwrap(),
                    sample_set: params.next().unwrap().parse().unwrap(),
                    sample_index: params.next().unwrap().parse().unwrap(),
                    volume: params.next().unwrap().parse().unwrap(),
                    uninherited: params.next().unwrap().parse::<usize>().unwrap() != 0,
                    effects: params.next().unwrap().parse().unwrap(),
                });
            }
            Some(Section::Colours(s)) => {
                s.push(Colour { 
                    combo: params.next().unwrap().trim_start_matches("Combo").parse().unwrap(),
                    slider_track_override: params.next().unwrap().parse().unwrap(),
                    slider_border: params.next().unwrap().parse().unwrap(),
                });
            }
            Some(Section::HitObjects(s)) => {
                s.push(HitObject { 
                    x: params.next().unwrap().parse().unwrap(),
                    y: params.next().unwrap().parse().unwrap(),
                    time: params.next().unwrap().parse().unwrap(),
                    kind: match params.next() {
                        Some("0") => HitObjectKind::HitCircle,
                        Some("1") => HitObjectKind::Slider,
                        Some("2") => HitObjectKind::Spinner,
                        Some("3") => HitObjectKind::ManiaHold,
                        _ => {return  Ok(());}//{return self.error("invalid hitcircle type")},
                    },
                    hit_sound: params.next().unwrap().parse().unwrap(),
                    object_params: match params.next() {
                        Some(string) => Some(String::from(string)),
                        None => None,
                    },
                    hit_sample: match params.next() {
                        Some(string) => Some(String::from(string)),
                        None => None,
                    },
                });
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_key(&mut self) -> Result<(), Error> {

        let key = self.parse_str_until(
            &[Some(b'='), Some(b':')]
        )?.trim().to_owned();

        if !key.is_empty() {
            self.key = Some(key);
        }

        Ok(())
    }

    fn parse_val(&mut self) -> Result<(), Error> {
        self.bump();
        self.parse_whitespace_except_line_break();

        let val;
        match self.chr {
            None => val = Ok(String::new()),
            _ => val = self.parse_str_until_eol()
        }

        let mval = val?;
        let mval = mval.trim();

        

        if let Some(sec) = self.section.as_mut(){
            //write value
            match sec {
                Section::General(s) => match self.key.as_mut() {
                    Some(k) => match k.as_str() {
                        "AudioFilename" => s.audio_filename = Some(PathBuf::from(mval)),
                        "AudioLeadIn" => s.audio_lead_in = Some(mval.parse().unwrap()),
                        "AudioHash" => s.audio_hash = Some(String::from(mval)),
                        "PreviewTime" => s.preview_time = Some(mval.parse().unwrap()),
                        "Countdown" => s.countdown = Some(mval.parse().unwrap()),
                        "SampleSet" => s.sample_set = Some(String::from(mval)),
                        "StackLeniency" => s.stack_leniency = Some(mval.parse().unwrap()),
                        "Mode" => s.mode = Some(mval.parse().unwrap()),
                        "LetterboxInBreaks" => s.letter_box_in_breaks = Some(mval.parse::<usize>().unwrap() != 0),
                        "StoryFireInFront" => s.story_fire_in_front = Some(mval.parse::<usize>().unwrap() != 0),
                        "UseSkinSprites" => s.use_skin_sprites = Some(mval.parse::<usize>().unwrap() != 0),
                        "AlwaysShowPlayfield" => s.always_show_playfield = Some(mval.parse::<usize>().unwrap() != 0),
                        "OverlayPosition" => s.overlay_position = Some(String::from(mval)),
                        "SkinPreference" => s.skin_preference = Some(String::from(mval)),
                        "EpilepsyWarning" => s.epilepsy_warning = Some(mval.parse::<usize>().unwrap() != 0),
                        "CountdownOffset" => s.countdown_offset = Some(mval.parse().unwrap()),
                        "SpecialStyle" => s.special_style = Some(mval.parse::<usize>().unwrap() != 0),
                        "WidescreenStoryboard" => s.widescreen_storyboard = Some(mval.parse::<usize>().unwrap() != 0),
                        "samples_match_playback_rate" => s.samples_match_playback_rate = Some(mval.parse::<usize>().unwrap() != 0),
                        _ => {}
                    }
                    None => return self.error("Key not defined"),
                }
                Section::Editor(s) => match self.key.as_mut() {
                    Some(k) => match k.as_str() {
                        "Bookmarks" => s.bookmarks = Some(String::from(mval)),
                        "DistanceSpacing" => s.distance_spacing = Some(mval.parse().unwrap()),
                        "BeatDivisor" => s.beat_divisor = Some(mval.parse().unwrap()),
                        "GridSize" => s.grid_size = Some(mval.parse().unwrap()),
                        "TimelineZoom" => s.timeline_zoom = Some(mval.parse().unwrap()),
                        _ => {}
                    }
                    None => return self.error("Key not defined"),
                }
                Section::Metadata(s) => match self.key.as_mut() {
                    Some(k) => match k.as_str() {
                        "Title" => s.title = Some(String::from(mval)),
                        "TitleUnicode" => s.title_unicode = Some(String::from(mval)),
                        "Artist" => s.artist = Some(String::from(mval)),
                        "ArtistUnicode" => s.artist_unicode = Some(String::from(mval)),
                        "Creator" => s.creator = Some(String::from(mval)),
                        "Version" => s.version = Some(String::from(mval)),
                        "Source" => s.source = Some(String::from(mval)),
                        "Tags" => s.tags = Some(String::from(mval)),
                        "BeatmapID" => s.beatmap_id = Some(mval.parse().unwrap()),
                        "BeatmapSetID" => s.beatmap_set_id = Some(mval.parse().unwrap()),
                        _ => {}
                    }
                    _ => {}
                }
                Section::Difficulty(s) => match self.key.as_mut() {
                    Some(k) => match k.as_str() {
                        "HPDrainRate" => s.hp_drain_rate = Some(mval.parse().unwrap()),
                        "CircleSize" => s.circle_size = Some(mval.parse().unwrap()),
                        "OverallDifficulty" => s.overall_difficulty = Some(mval.parse().unwrap()),
                        "ApproachRate" => s.approach_rate = Some(mval.parse().unwrap()),
                        "SliderMultiplier" => s.slider_multiplier = Some(mval.parse().unwrap()),
                        "SliderTickRate" => s.slider_tick_rate = Some(mval.parse().unwrap()),
                        _ => {}
                    }
                    _ => {}
                }
                _ => {}
            }
        }
        self.key = None;
        return Ok(())
    }

    fn parse_str_until_eol(&mut self) -> Result<String, Error> {
        self.parse_str_until(&[Some(b'\n'), Some(b'\r'), None])
    }

    fn finish_section(&mut self, section: Section) -> Result<(), Error>{

        
        if let Some(res) = self.result.as_mut(){

            match section {
            
                Section::General(s) => res.general = Some(s),
                Section::Editor(s) => res.editor = Some(s),
                Section::Metadata(s) => res.metadata = Some(s),
                Section::Difficulty(s) => res.difficulty = Some(s),
                Section::Events(s) => res.events = Some(s),
                Section::TimingPoints(s) => res.timing_points = Some(s),
                Section::Colours(s) => res.colours = Some(s),
                Section::HitObjects(s) => res.hit_objects = Some(s),
            }

        } else {
            return self.error("No Result");
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    Parse(String)
}
