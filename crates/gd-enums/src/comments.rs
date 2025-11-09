#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum State {
    #[default]
    OpenToAll = 0,
    OpenToFriends = 1,
    Closed = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Type {
    User = 0,
    Level = 1,
}
