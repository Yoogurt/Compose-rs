#[derive(Debug, Clone, PartialEq)]
pub struct IntRect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl IntRect {
    pub const ZERO: IntRect = IntRect {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn width(&self) -> usize {
        (self.right - self.left) as usize
    }

    pub fn height(&self) -> usize {
        (self.bottom - self.top) as usize
    }

    pub fn size(&self) -> IntSize {
        IntSize::new(self.width(), self.height())
    }

    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    pub fn translate(&self, offset: IntOffset) -> Self {
        Self {
            left: self.left + offset.x,
            top: self.top + offset.y,
            right: self.right + offset.x,
            bottom: self.bottom + offset.y,
        }
    }

    pub fn translate_x_y(&self, x: i32, y: i32) -> Self {
        Self {
            left: self.left + x,
            top: self.top + y,
            right: self.right + x,
            bottom: self.bottom + y,
        }
    }

    pub fn inflate(&self, delta: i32) -> Self {
        Self {
            left: self.left - delta,
            top: self.top - delta,
            right: self.right + delta,
            bottom: self.bottom + delta,
        }
    }

    pub fn deflate(&self, delta: i32) -> Self {
        self.inflate(-delta)
    }

    pub fn intersect(&self, other: &Self) -> Self {
        Self {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        if self.right <= other.left || other.right <= self.left {
            return false;
        }
        if self.bottom <= other.top || other.bottom <= self.top {
            return false;
        }

        true
    }

    pub fn contains(&self, point: IntOffset) -> bool {
        self.left <= point.x && point.x < self.right && self.top <= point.y && point.y < self.bottom
    }


    pub fn top_left(&self) -> IntOffset {
        IntOffset::new(self.left, self.top)
    }

    pub fn top_right(&self) -> IntOffset {
        IntOffset::new(self.right, self.top)
    }

    pub fn bottom_left(&self) -> IntOffset {
        IntOffset::new(self.left, self.bottom)
    }

    pub fn bottom_right(&self) -> IntOffset {
        IntOffset::new(self.right, self.bottom)
    }

    pub fn top_center(&self) -> IntOffset {
        IntOffset::new((self.left + self.right) / 2, self.top)
    }

    pub fn bottom_center(&self) -> IntOffset {
        IntOffset::new((self.left + self.right) / 2, self.bottom)
    }

    pub fn left_center(&self) -> IntOffset {
        IntOffset::new(self.left, (self.top + self.bottom) / 2)
    }

    pub fn right_center(&self) -> IntOffset {
        IntOffset::new(self.right, (self.top + self.bottom) / 2)
    }

    pub fn center(&self) -> IntOffset {
        IntOffset::new((self.left + self.right) / 2, (self.top + self.bottom) / 2)
    }
}