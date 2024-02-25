use std::{marker::PhantomData, rc::Rc};

use string_reader::{StringRead, StringReader, StringWrite};

use super::{line::Single, Application, Drawable, Intersect, LineMake};

/// A box-like rectangle container.
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect<Border: LineMake = Single> {
    pub size: (usize, usize),
    pub coor: (usize, usize),
    border: PhantomData<Border>,
}

impl<B: LineMake + 'static> Drawable for Rect<B> {
    fn draw(&self, app: &Application) -> StringReader {
        // top/bottom left/right corners
        let (mut tl, mut tr) = (Intersect::new(), Intersect::new());
        let (mut bl, mut br) = (Intersect::new(), Intersect::new());
        // horz/vert lines
        let (hl, vl) = (Rc::new(B::new_horz()), Rc::new(B::new_vert()));
        bl.up = Some(vl.clone());
        br.up = Some(vl.clone());
        tl.down = Some(vl.clone());
        tr.down = Some(vl.clone());
        tr.left = Some(hl.clone());
        bl.left = Some(hl.clone());
        tl.right = Some(hl.clone());
        bl.right = Some(hl.clone());

        let mut ret = StringReader::new();
        let horz = hl.chr().to_string().repeat(self.size().0 - 2);
        ret.push_string(app.must_handle_intersect(&tl).into());
        ret.push_string(horz.clone());
        ret.push_string(app.must_handle_intersect(&tr).into());
        ret.push_string("\n".into());
        while let Some(vert) = vl.generate(self.size().1 - 2).pop_string() {
            ret.push_string(vert.clone());
            ret.push_string(" ".repeat(self.size().0 - 2).to_string());
            ret.push_string(vert);
            ret.push_string("\n".to_string());
        }
        ret.push_string(app.must_handle_intersect(&bl).into());
        ret.push_string(horz);
        ret.push_string(app.must_handle_intersect(&br).into());
        ret
    }

    fn coor(&self) -> (usize, usize) {
        self.coor
    }

    fn size(&self) -> (usize, usize) {
        self.size
    }
}

/// A box-like rectangle container without borders.
pub struct RectBorderless {
    pub size: (usize, usize),
    pub coor: (usize, usize),
}
