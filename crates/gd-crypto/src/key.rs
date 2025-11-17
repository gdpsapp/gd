pub type Data<const S: usize> = [u8; S];

#[derive(Debug, Clone, Copy)]
pub struct Key<const S: usize> {
    data: Data<S>,
}

impl<const S: usize> Key<S> {
    pub const fn new(data: Data<S>) -> Self {
        Self { data }
    }

    pub const fn get(self) -> Data<S> {
        self.data
    }
}
