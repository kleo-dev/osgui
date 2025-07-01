use crate::component;

pub struct RawTransform {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub enum Position {
    Const(usize),
    Center,
    End,
}

#[derive(Debug, Clone)]
pub enum Dimension {
    Auto,
    Const(usize),
}

component!(Transform {
    pub x: Position,
    pub y: Position,
    pub mx: i32,
    pub my: i32,
    pub width: Dimension,
    pub height: Dimension,
});

impl Transform {
    pub fn new() -> Transform {
        Transform {
            x: Position::Const(0),
            y: Position::Const(0),
            mx: 0,
            my: 0,
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }

    pub fn center() -> Transform {
        Transform {
            x: Position::Center,
            y: Position::Center,
            mx: 0,
            my: 0,
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }

    pub fn bottom(mut self) -> Self {
        self.y = Position::End;
        self
    }

    pub fn margin(mut self, x: i32, y: i32) -> Self {
        self.mx = x;
        self.my = y;
        self
    }

    pub fn dimensions(mut self, width: usize, height: usize) -> Self {
        self.width = Dimension::Const(width);
        self.height = Dimension::Const(height);
        self
    }

    pub fn use_dimensions(&self, raw: &mut RawTransform) {
        self.width.use_dimension(&mut raw.width);
        self.height.use_dimension(&mut raw.height);
    }

    pub fn use_position(&self, parent_width: usize, parent_height: usize, raw: &mut RawTransform) {
        self.x
            .use_position(raw.width, parent_width, self.mx, &mut raw.x);
        self.y
            .use_position(raw.height, parent_height, self.my, &mut raw.y);
    }
}

impl RawTransform {
    pub fn new() -> RawTransform {
        RawTransform {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

impl Dimension {
    pub fn use_dimension(&self, r: &mut usize) {
        match self {
            Self::Auto => {}
            Self::Const(n) => *r = *n,
        }
    }
}

impl Position {
    pub fn use_position(&self, size: usize, parent: usize, m: i32, r: &mut usize) {
        match self {
            Self::Center => *r = (parent - size) / 2,
            Self::Const(n) => *r = *n,
            Self::End => *r = parent - size,
        }

        if m > 0 {
            *r += m as usize;
        } else {
            *r -= -m as usize;
        }
    }
}
