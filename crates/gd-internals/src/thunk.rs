use std::{
    error::Error,
    hash::{Hash, Hasher},
    mem::{discriminant, replace},
};

pub trait Empty {
    fn empty() -> Self;
}

pub fn take<T: Empty>(value: &mut T) -> T {
    replace(value, T::empty())
}

pub trait DefaultIsEmpty: Default {}

impl<T: DefaultIsEmpty> Empty for T {
    fn empty() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub enum Thunk<'a, P: Processor> {
    Unprocessed(P::Unprocessed<'a>),
    Processed(P::Processed<'a>),
}

impl<'a, 'b, P: Processor> PartialEq<Thunk<'b, P>> for Thunk<'a, P>
where
    P::Unprocessed<'a>: PartialEq<P::Unprocessed<'b>>,
    P::Processed<'a>: PartialEq<P::Processed<'b>>,
{
    fn eq(&self, other: &Thunk<'b, P>) -> bool {
        match (self, other) {
            (Self::Unprocessed(self_unprocessed), Thunk::Unprocessed(other_unprocessed)) => {
                self_unprocessed == other_unprocessed
            }
            (Self::Processed(self_processed), Thunk::Processed(other_processed)) => {
                self_processed == other_processed
            }
            _ => false,
        }
    }
}

impl<'a, P: Processor> Eq for Thunk<'a, P>
where
    P::Unprocessed<'a>: PartialEq,
    P::Processed<'a>: PartialEq,
{
}

impl<'a, P: Processor> Hash for Thunk<'a, P>
where
    P::Unprocessed<'a>: Hash,
    P::Processed<'a>: Hash,
{
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        discriminant(self).hash(hasher);

        match self {
            Self::Unprocessed(unprocessed) => {
                unprocessed.hash(hasher);
            }
            Self::Processed(processed) => {
                processed.hash(hasher);
            }
        }
    }
}

pub trait Processor {
    type Unprocessed<'u>;
    type Processed<'p>;

    type ProcessError: Error;
    type UnprocessError: Error;

    fn process<'a>(
        unprocessed: Self::Unprocessed<'a>,
    ) -> Result<Self::Processed<'a>, Self::ProcessError>;

    fn unprocess<'a>(
        processed: Self::Processed<'a>,
    ) -> Result<Self::Unprocessed<'a>, Self::UnprocessError>;
}

impl<'a, P: Processor> Thunk<'a, P> {
    pub const fn is_processed(&self) -> bool {
        matches!(self, Self::Processed(_))
    }

    pub const fn is_unprocessed(&self) -> bool {
        matches!(self, Self::Unprocessed(_))
    }

    pub fn into_processed(self) -> Result<P::Processed<'a>, P::ProcessError> {
        match self {
            Self::Unprocessed(input) => P::process(input),
            Self::Processed(output) => Ok(output),
        }
    }

    pub fn into_unprocessed(self) -> Result<P::Unprocessed<'a>, P::UnprocessError> {
        match self {
            Self::Processed(output) => P::unprocess(output),
            Self::Unprocessed(input) => Ok(input),
        }
    }
}

impl<'a, P: Processor> Thunk<'a, P>
where
    P::Unprocessed<'a>: Empty,
{
    pub fn process(&mut self) -> Result<&mut P::Processed<'a>, P::ProcessError> {
        if let Self::Unprocessed(unprocessed) = self {
            let processed = P::process(take(unprocessed))?;

            *self = Self::Processed(processed);
        };

        match self {
            Self::Processed(processed) => Ok(processed),
            Self::Unprocessed(_) => unreachable!(),
        }
    }
}

impl<'a, P: Processor> Thunk<'a, P>
where
    P::Processed<'a>: Empty,
{
    pub fn unprocess(&mut self) -> Result<&mut P::Unprocessed<'a>, P::UnprocessError> {
        if let Self::Processed(processed) = self {
            let unprocessed = P::unprocess(take(processed))?;

            *self = Self::Unprocessed(unprocessed);
        };

        match self {
            Self::Unprocessed(unprocessed) => Ok(unprocessed),
            Self::Processed(_) => unreachable!(),
        }
    }
}
