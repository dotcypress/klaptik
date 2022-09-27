#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Point(pub u16, pub u16);

impl Point {
    pub const fn x(&self) -> u16 {
        self.0
    }

    pub const fn y(&self) -> u16 {
        self.1
    }
}

impl From<(u16, u16)> for Point {
    fn from(point: (u16, u16)) -> Self {
        Self(point.0, point.1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Size(pub u16, pub u16);

impl Size {
    pub const fn width(&self) -> u16 {
        self.0
    }

    pub const fn height(&self) -> u16 {
        self.1
    }
}

impl From<(u16, u16)> for Size {
    fn from(size: (u16, u16)) -> Self {
        Self(size.0, size.1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Rect(pub Point, pub Size);

impl Rect {
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self(Point(x, y), Size(width, height))
    }

    pub const fn origin(&self) -> Point {
        self.0
    }

    pub const fn size(&self) -> Size {
        self.1
    }
}

impl From<(u16, u16, u16, u16)> for Rect {
    fn from(size: (u16, u16, u16, u16)) -> Self {
        Self((size.0, size.1).into(), (size.2, size.3).into())
    }
}
