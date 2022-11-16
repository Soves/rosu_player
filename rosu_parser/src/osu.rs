use std::{str::{Chars, Bytes}, collections::{HashMap, btree_map::VacantEntry}, rc::Rc, cell::RefCell, path::{Path, PathBuf}, fs};

//TODO: 
// value parsing into int/str
// comma separated value lists
// parsing version header

#[derive(Debug)]
pub struct Osu {
    pub sections: HashMap<String, Properties>
}

impl Osu {
    
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load_from_string(string: String) -> Result<Osu, Error> {
        let mut parser = Parser::new(string.bytes());
        parser.parse()
    }

    pub fn load_from_file(filename: PathBuf) -> Result<Osu, Error> {
        Osu::load_from_string(fs::read_to_string(filename).unwrap())
    }

    //TODO: handle gracefully
    pub fn get(&self, section: String, key: String) -> &Vec<String>{
        self.sections.get(&section).unwrap().data.get(&key).unwrap()
    }

}

impl Default for Osu {
    fn default() -> Self {
        Self { sections: Default::default() }
    }
}

#[derive(Debug)]
pub struct Properties {
    data: HashMap<String, Vec<String>>
}

impl Properties {
    pub fn new() -> Self {
        Self { data: Default::default() }
    }
}

#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(i32),
}

struct Parser<'a> {
    chr: Option<u8>,  //curent char
    reader: Bytes<'a>,  //char iterator
    line: usize,        //current line
    col: usize          //currennt column
}

impl<'a> Parser<'a> {

    fn new(reader: Bytes<'a>) -> Self {
        let mut parser = Self {
            chr: None,
            reader,
            line: 0,
            col: 0
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

    fn parse(&mut self) -> Result<Osu, Error> {
        
        let mut result = Osu::new();
        let mut curkey: String = "".into();
        let mut cursec: Option<String> = None;

        self.parse_whitespace();

        while let Some(cur_chr) = self.chr {
            match cur_chr {
                b';' | b'#' => {
                    if self.col > 1 {
                        return self.error("doesn't supprt inline comments");
                    }

                    self.parse_comment();
                }
                b'[' => match self.parse_section() {
                    Ok(sec) => {
                        let msec = sec[..].trim();
                        cursec = Some((*msec).to_string());

                        if let Some(sec) = cursec.clone() {

                            if result.sections.contains_key(&sec) {
                                return self.error("duplicate section entry");
                            }
                            
                            let prop = Properties::new();
                            result.sections.insert(sec, prop);
                        }

                        self.bump()
                    }
                    Err(e) => return Err(e),
                },
                b'=' | b':' => {
                    if (&curkey[..]).is_empty() {
                        return self.error("missing key");
                    }

                    match self.parse_val() {
                        Ok(val) => {
                            let mval = val[..].trim().to_owned();
                            
                            if let Some(sec) = cursec.clone() {
                                if let Some(prop) = result.sections.get_mut(&sec){
                                    
                                    //section exists
                                    prop.data.insert(curkey, 
                                        vec![String::from(mval)] );
                                }
                            }
                            curkey = "".into();
                        }
                        Err(e) => return Err(e),
                    }
                }
                _ => match self.parse_key() {
                    Ok(key) => {
                        let mkey: String = key[..].trim().to_owned();
                        curkey = mkey;
                    }
                    Err(e) => return Err(e),
                }
            }

            self.parse_whitespace();
        }

        Ok(result)
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

    fn parse_section(&mut self) -> Result<String, Error>{
        self.bump();
        self.parse_str_until(&[Some(b']')])
    }

    fn parse_key(&mut self) -> Result<String, Error> {
        self.parse_str_until(&[Some(b'='), Some(b':')])
    }

    fn parse_val(&mut self) -> Result<String, Error> {
        self.bump();
        self.parse_whitespace_except_line_break();

        match self.chr {
            None => Ok(String::new()),
            _ => self.parse_str_until_eol()
        }
    }

    fn parse_str_until_eol(&mut self) -> Result<String, Error> {
        self.parse_str_until(&[Some(b'\n'), Some(b'\r'), None])
    }
}

#[derive(Debug)]
pub enum Error {
    Parse(String)
}
