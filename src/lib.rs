//! This crate provides a converter from
//! [SDF](https://datatracker.ietf.org/doc/html/draft-ietf-asdf-sdf-05)
//! (including protocol bindings) to [WoT TD](https://www.w3.org/TR/wot-thing-description/).
//!
//! The converter is both usable as a library that can be built upon in other WoT
//! and SDF related projects as well as a tool for the command line.

use std::{error, result};

pub mod converter;
pub mod sdf;
pub mod wot;
pub mod converters;

pub type Result<T> = result::Result<T, Error>;
type Error = Box<dyn error::Error>;
