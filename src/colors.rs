/// The charater used in ANSI escape sequences.
pub const ESC: char = 0x1b as char;

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const ITALIC: &str = "\x1b[3m";
pub const URL: &str = "\x1b[4m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const BLINK: &str = "\x1b[5m";
pub const BLINK2: &str = "\x1b[6m";
pub const SELECTED: &str = "\x1b[7m";

pub mod fg {
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const VIOLET: &str = "\x1b[35m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";

    pub const GREY: &str = "\x1b[90m";
    pub const RED2: &str = "\x1b[91m";
    pub const GREEN2: &str = "\x1b[92m";
    pub const YELLOW2: &str = "\x1b[93m";
    pub const BLUE2: &str = "\x1b[94m";
    pub const VIOLET2: &str = "\x1b[95m";
    pub const BEIGE2: &str = "\x1b[96m";
    pub const WHITE2: &str = "\x1b[97m";

    pub const ORANGE: &str = "\x1b[38;2;255;135;0m";
}
pub mod bg {
    pub const BLACK: &str = "\x1b[40m";
    pub const RED: &str = "\x1b[41m";
    pub const GREEN: &str = "\x1b[42m";
    pub const YELLOW: &str = "\x1b[43m";
    pub const BLUE: &str = "\x1b[44m";
    pub const VIOLET: &str = "\x1b[45m";
    pub const BEIGE: &str = "\x1b[46m";
    pub const WHITE: &str = "\x1b[47m";

    pub const GREY: &str = "\x1b[100m";
    pub const RED2: &str = "\x1b[101m";
    pub const GREEN2: &str = "\x1b[102m";
    pub const YELLOW2: &str = "\x1b[103m";
    pub const BLUE2: &str = "\x1b[104m";
    pub const VIOLET2: &str = "\x1b[105m";
    pub const BEIGE2: &str = "\x1b[106m";
    pub const WHITE2: &str = "\x1b[107m";

    pub const ORANGE: &str = "\x1b[48;2;255;135;0m";
}

#[cfg(test)]
mod test {
    #[test]
    fn colors() {
        use crate::colors;
        println!("{}hai{}", colors::fg::RED, colors::RESET);
    }
}
