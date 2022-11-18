use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Background {
    pub start_time: usize,
    pub filename: PathBuf,
    pub x_offset: isize,
    pub y_offset: isize,
}
#[derive(Debug, Default)]
pub struct Video {
    pub start_time: usize,
    pub filename: PathBuf,
    pub x_offset: isize,
    pub y_offset: isize,
}
#[derive(Debug, Default)]
pub struct Break {
    pub start_time: usize,
    pub end_time: String,
}