use crate::foundation::LayoutResult;

impl LayoutResult {
    pub(crate) const fn from_ltrb(left: i32,
                                  top: i32,
                                  right: i32,
                                  bottom: i32) -> LayoutResult {
        LayoutResult {
            left,
            top,
            right,
            bottom,
        }
    }

    pub(crate) const fn from_ltwh(left: i32,
                                  top: i32,
                                  width: i32,
                                  height: i32) -> LayoutResult {
        Self::from_ltrb(left, top, left + width, top + height)
    }
}