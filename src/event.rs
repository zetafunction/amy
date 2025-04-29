#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Read,
    Write,
    Both,
}

impl Event {
    pub fn readable(&self) -> bool {
        matches!(self, Event::Read | Event::Both)
    }

    pub fn writable(&self) -> bool {
        matches!(self, Event::Write | Event::Both)
    }
}
