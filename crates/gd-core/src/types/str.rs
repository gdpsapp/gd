use std::borrow::Cow;

pub type Str<'s> = Cow<'s, str>;
