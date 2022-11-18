use std::{str::{Chars, Bytes}, collections::{HashMap, btree_map::VacantEntry}, rc::Rc, cell::RefCell, path::{Path, PathBuf}, fs};

//TODO: 
// value parsing into int/str
// comma separated value lists
// parsing version header

#[derive(Debug)]
pub struct Osu {
    pub general: Option<General>,
    pub editor: Option<Editor>,
    pub metadata: Option<Metadata>,
    pub difficulty: Option<Difficulty>,
    pub events: Option<Events>,
    pub timing_points: Option<TimingPoints>,
    pub colours: Option<Colours>,
    pub hit_objects: Option<HitObjects>,
}

impl Osu {
    
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load_from_string(string: String) -> Result<Osu, Error> {
        let mut parser = Parser::new(string.bytes());
        parser.parse()
    }

    pub fn load_from_file(filename: &PathBuf) -> Result<Osu, Error> {
        Osu::load_from_string(fs::read_to_string(filename).unwrap())
    }

    //TODO: handle gracefully
    pub fn get(&self, section: String, key: String) -> String{
        todo!()
        //self.sections.get(&section).unwrap().data.get(&key).unwrap().into()
    }
}

impl Default for Osu {
    fn default() -> Self {
        Self { 
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

#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i32),
}

struct Parser<'a> {
    chr: Option<u8>,          //curent char
    section: Option<Section>, //current section
    key: Option<String>,      //current key
    reader: Bytes<'a>,        //char iterator
    line: usize,              //current line
    col: usize,               //currennt column
    result: Option<Osu>,
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

    fn parse_header(&mut self) -> Result<String, Error>{
        self.parse_whitespace();
        let res = self.parse_str_until_eol();
        self.parse_whitespace();
        res
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

    fn parse(&mut self) -> Result<Osu, Error> {
        
        self.result = Some(Osu::new());
        self.key = None;

        let _header = self.parse_header()?;

        while let Some(cur_chr) = self.chr {
            match cur_chr {
                b';' | b'#' => {
                    if self.col > 1 {
                        return self.error("doesn't supprt inline comments");
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
                _ => self.parse_key()?,
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
        match section_str {
        
            "General" => next_section = Section::General(General::default()),
            "Editor" => next_section = Section::Editor(Editor::default()),
            "Metadata" => next_section = Section::Metadata(Metadata::default()),
            "Difficulty" => next_section= Section::Difficulty(Difficulty::default()),
            "Events" => next_section = Section::Events(Events::default()),
            "TimingPoints" => next_section = Section::TimingPoints(TimingPoints::default()),
            "Colours" => next_section = Section::Colours(Colours::default()),
            "HitObjects" => next_section = Section::HitObjects(HitObjects::default()),

            _ => {
                //undefined section
                self.section = None; //reset section so we dont write to previous section
                return self.error(format!(
                    "Undefined section \"{}\"", 
                    section_str
                ));
            }
        }
        
        if let Some(sec) = self.section.take() {
            self.finish_section(sec)?;
        }
        self.section = Some(next_section);
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
        
                Section::General(s) => {
                    
                    match self.key.as_mut() {
                        Some(k) => {
                            
                            match k.as_str() {
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
                           
                        }
                        None => return self.error("Key not defined"),
                    }
                }
                Section::Editor(s) => {

                }
                Section::Metadata(s) => {
                    match self.key.as_mut() {
                        Some(k) => {
                            
                            match k.as_str() {
                                "Title" => s.title = Some(String::from(mval)),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                Section::Difficulty(s) => {
                    
                }
                Section::Events(s) => {
                    
                }
                Section::TimingPoints(s) => {
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
