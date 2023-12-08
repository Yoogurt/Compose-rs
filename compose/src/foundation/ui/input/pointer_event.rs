#[derive(Clone, PartialEq, Debug, Copy)]
pub(crate) enum PointerEventType {
    Unknown = 0,
    Press = 1,
    Release = 2,
    Move = 3,
    Enter = 4,
    Exit = 5,
    Scroll = 6,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub(crate) enum PointerType {
    Unknown = 0,
    Touch = 1,
    Mouse = 2,
    Stylus = 3,
    Eraser = 4,
}