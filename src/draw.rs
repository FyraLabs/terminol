pub mod containers;
pub mod line;

use std::rc::Rc;
use string_reader::StringReader;

#[derive(Clone, Debug, Default)]
pub struct Intersect {
    pub up: Option<Rc<dyn Line>>,
    pub down: Option<Rc<dyn Line>>,
    pub left: Option<Rc<dyn Line>>,
    pub right: Option<Rc<dyn Line>>,
    pub props: Vec<String>,
}

impl Intersect {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn is_horz(&self) -> bool {
        matches!(self, Orientation::Horizontal)
    }
    pub fn is_vert(&self) -> bool {
        matches!(self, Orientation::Vertical)
    }
}

pub trait Line: std::fmt::Debug {
    fn get_ident(&self) -> String;
    fn get_orientation(&self) -> Orientation;
    fn chr(&self) -> char;
    /// Generates the line.
    ///
    /// If your line is vertical, you need [`StringWrite::push_string`] for each line.
    /// If you line is horizontal, you need to `push_string` everything in one go.
    fn generate(&self, length: usize) -> StringReader {
        if self.get_orientation().is_horz() {
            self.chr().to_string().repeat(length).into()
        } else {
            (0..length)
                .map(|_| self.chr().to_string())
                .collect::<std::collections::VecDeque<_>>()
                .into()
        }
    }
}

pub trait LineMake: Line {
    fn new_horz() -> Self;
    fn new_vert() -> Self;
}

/// Returns a char for the line intersect
pub type IntersectHandler = Rc<dyn Fn(&Intersect) -> Option<char>>;

#[rustfmt::skip]
pub fn terminol_default_intersect_handler(int: Intersect) -> Option<char> {
    fn mapper(s: Option<Rc<dyn Line>>, shift: u8) -> Option<u8> {
        Some(
            match s.map(|s| s.get_ident()).as_ref().map(|s| s.as_str()) {
                Some("single") => 0b00,
                Some("double") => 0b01,
                Some("bold") => 0b10,
                None => 0b11,
                _ => return None, // we don't handle "ascii" here
            } >> shift * 2,
        )
    }
    let mut flags: u8 = 0;
    flags |= mapper(int.up, 0)?;
    flags |= mapper(int.down, 1)?;
    flags |= mapper(int.left, 2)?;
    flags |= mapper(int.right, 3)?;
    Some([
    //  lr   lr   lr   lr   lr   lr   lr   lr   lr   lr   lr   lr   lr   lr   lr   lr    ┃
    // ──────────────────────────────────────────────────────────────────────────────────┨ uuddllrr
    //  ss   sd   sb   s_   ds   dd   db   d_   bs   bd   bb   b_   _s   _d   _b   __    ┗━━━━━━━━━
        '┼', '╪', '┾', '┤', '╪', '╪', '╪', '╡', '┽', '╪', '┿', '┥', '├', '╞', '┝', '│', // 0000....
        '╫', '╬', '╫', '╢', '╬', '╬', '╬', '╣', '╫', '╬', '╫', '╢', '╟', '╠', '╟', '║', // 0001....
        '╁', '╪', '╆', '┧', '╪', '╪', '╪', '╡', '┽', '╪', '╈', '┪', '┟', '╞', '┢', '╽', // 0010....
        '┴', '╧', '┶', '┘', '╧', '╧', '╧', '╛', '┵', '╧', '┷', '┙', '└', '╘', '┕', '╵', // 0011....
        '╫', '╬', '╬', '╢', '╬', '╬', '╬', '╣', '╫', '╬', '╫', '╢', '╟', '╠', '╟', '║', // 0100....
        '╫', '╬', '╫', '╢', '╬', '╬', '╬', '╣', '╫', '╬', '╫', '╢', '╟', '╠', '╟', '║', // 0101....
        '╫', '╬', '╫', '╢', '╬', '╬', '╬', '╣', '╫', '╬', '╫', '╢', '╟', '╠', '╟', '║', // 0110....
        '╨', '╩', '╨', '╜', '╩', '╩', '╩', '╝', '╨', '╩', '╨', '╜', '╙', '╚', '╙', '╹', // 0111....
        '╀', '╪', '╄', '┦', '╪', '╪', '╪', '╡', '╃', '╪', '╇', '┩', '┞', '╞', '┡', '╿', // 1000....
        '╫', '╬', '╫', '╢', '╬', '╬', '╬', '╣', '╫', '╬', '╫', '╢', '╟', '╠', '╟', '║', // 1001....
        '╂', '╪', '╊', '┨', '╪', '╪', '╪', '╡', '╉', '╪', '╋', '┫', '┠', '╞', '┣', '┃', // 1010....
        '┸', '╧', '┺', '┚', '╧', '╧', '╧', '╛', '┹', '╧', '┻', '┛', '┖', '╘', '┗', '╹', // 1011....
        '┬', '╤', '┮', '┐', '╤', '╤', '╤', '╕', '┭', '╤', '┯', '┑', '┌', '╒', '┍', '╷', // 1100....
        '╥', '╦', '╥', '╖', '╦', '╦', '╦', '╗', '╥', '╦', '╥', '╖', '╓', '╔', '╓', '╻', // 1101....
        '┰', '╤', '┲', '┒', '╤', '╤', '╤', '╕', '┱', '╤', '┳', '┓', '┎', '╒', '┏', '╻', // 1110....
        '─', '═', '╼', '╴', '═', '═', '═', '╸', '╾', '═', '━', '╸', '╶', '╺', '╺', ' ', // 1111....
    ][flags as usize])
}

pub trait Drawable: std::fmt::Debug {
    /// Return a [`StringReader`] with the final drawn text.
    fn draw(&self, app: &Application) -> StringReader;
    /// Returns an (x, y) coordinate.
    ///
    /// Coordinates are defined where (0, 0) is the top left corner of a terminal, and each block
    /// on the right / below the x/y value increases by 1.
    fn coor(&self) -> (usize, usize);
    /// Returns an (x, y) size of the drawable.
    fn size(&self) -> (usize, usize);
    // /// Returns a list of [`Intersect`].
    // fn get_intersects(&self) -> Box<[Intersect]>;
}

#[derive(Default)]
pub struct Application {
    pub intersect_handlers: Vec<IntersectHandler>,
    pub objs: Vec<Box<dyn Drawable>>,
}

impl Application {
    pub fn new() -> Self {
        // TODO: default line_handlers
        Self::default()
    }

    pub fn handle_intersect(&self, int: &Intersect) -> Option<char> {
        self.intersect_handlers.iter().find_map(|hdl| hdl(int))
    }

    pub fn must_handle_intersect(&self, int: &Intersect) -> char {
        self.handle_intersect(&int)
            .expect(&format!("Cannot handle intersect {int:?}"))
    }
}
