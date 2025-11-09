use gd_internals::session::Session;

use crate::{
    auth::{Auth, Credentials},
    state::{self, State},
};

pub struct Client<S: State> {
    pub session: Session,
    pub state: S,
}

impl<S: State> Client<S> {
    pub const fn new(session: Session, state: S) -> Self {
        Self { session, state }
    }
}

pub struct LoginError {
    pub client: Simple,
}

pub type Simple = Client<state::Simple>;

impl Simple {
    pub const fn simple(session: Session) -> Self {
        Self::new(session, state::Simple::new())
    }

    pub fn authenticate(self, auth: Auth<'_>) -> Authenticated<'_> {
        Authenticated::authenticated(self.session, auth)
    }

    pub async fn login<'c>(self, credentials: Credentials<'c>) -> Authenticated<'c> {
        todo!()
    }
}

impl<'a> Authenticated<'a> {
    pub const fn authenticated(session: Session, auth: Auth<'a>) -> Self {
        Self::new(session, state::Authenticated::new(auth))
    }

    pub fn simplify(self) -> Simple {
        Simple::simple(self.session)
    }
}

pub type Authenticated<'a> = Client<state::Authenticated<'a>>;
