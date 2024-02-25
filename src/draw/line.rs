#[macro_export]
macro_rules! impl_with_orientation_for_line_struct {
    ($($struct:ty),+) => {$(
        impl $crate::draw::LineMake for $struct {
            fn new_horz() -> Self {
                Self { orientation: $crate::draw::Orientation::Horizontal, ..Default::default() }
            }
            fn new_vert() -> Self {
                Self { orientation: $crate::draw::Orientation::Vertical, ..Default::default() }
            }
        }
    )+};
}

#[macro_export]
macro_rules! generate_line_struct {
    ($($v:vis $struct:ident($ident:expr) => $horz:literal $vert:literal)+) => {
        $crate::generate_line_struct!(orientation $($v $struct($ident) => $horz $vert)+);
    };
    ($orientation:ident $($v:vis $struct:ident($ident:expr) => $horz:literal $vert:literal)+) => {
        $(
            #[derive(Default, Debug, Clone)]
            $v struct $struct  {
                $v $orientation: $crate::draw::Orientation,
            }
            impl $crate::draw::Line for $struct {
                fn get_ident(&self) -> String {
                    $ident.to_string()
                }
                fn get_orientation(&self) -> $crate::draw::Orientation {
                    self.$orientation
                }
                fn chr(&self) -> char {
                    if self.$orientation.is_horz() { $horz } else { $vert }
                }
            }
        )+
        $crate::impl_with_orientation_for_line_struct!($($struct),+);
    };
}

generate_line_struct!(
    pub Single("single") => '─' '│'
    pub Double("double") => '═' '║'
    pub Bold("bold") => '━' '┃'
    pub Ascii("ascii") => '-' '|'
);
