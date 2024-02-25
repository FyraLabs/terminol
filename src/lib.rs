#[warn(clippy::pedantic)]
#[warn(clippy::nursery)]
#[cfg(feature = "cmds")]
pub mod cmds;
#[cfg(feature = "colors")]
pub mod colors;
#[cfg(feature = "draw")]
pub mod draw;
