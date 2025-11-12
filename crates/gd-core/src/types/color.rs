use std::{fmt, hash::Hash};

use bon::Builder;
use ownership::IntoOwned;

// TODO: (de)serialization

pub const ZERO: u8 = 0x00;
pub const FULL: u8 = 0xff;

pub type Color = OpaqueColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Builder, IntoOwned)]
pub struct OpaqueColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for OpaqueColor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.into_rgb();

        write!(formatter, "#{r:02x}{g:02x}{b:02x}")
    }
}

impl OpaqueColor {
    pub const BLACK: Self = Self::from_rgb(ZERO, ZERO, ZERO);
    pub const WHITE: Self = Self::from_rgb(FULL, FULL, FULL);

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn into_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub const fn from_array([r, g, b]: [u8; 3]) -> Self {
        Self::from_rgb(r, g, b)
    }

    pub const fn into_array(self) -> [u8; 3] {
        let (r, g, b) = self.into_rgb();

        [r, g, b]
    }

    pub const fn with_alpha(self, a: u8) -> AlphaColor {
        let (r, g, b) = self.into_rgb();

        AlphaColor::from_rgba(r, g, b, a)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Builder, IntoOwned)]
pub struct AlphaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Display for AlphaColor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b, a) = self.into_rgba();

        write!(formatter, "#{r:02x}{g:02x}{b:02x}{a:02x}")
    }
}

impl AlphaColor {
    pub const EMPTY: Self = Self::from_rgba(ZERO, ZERO, ZERO, ZERO);
    pub const BLACK: Self = Self::from_rgba(ZERO, ZERO, ZERO, FULL);
    pub const WHITE: Self = Self::from_rgba(FULL, FULL, FULL, FULL);

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn into_rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub const fn from_array([r, g, b, a]: [u8; 4]) -> Self {
        Self::from_rgba(r, g, b, a)
    }

    pub const fn into_array(self) -> [u8; 4] {
        let (r, g, b, a) = self.into_rgba();

        [r, g, b, a]
    }

    pub const fn split(self) -> (OpaqueColor, u8) {
        let (r, g, b, a) = self.into_rgba();

        (OpaqueColor::from_rgb(r, g, b), a)
    }

    pub const fn without_alpha(self) -> OpaqueColor {
        let (opaque, _) = self.split();

        opaque
    }
}
