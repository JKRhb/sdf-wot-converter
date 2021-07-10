//! This crate provides a converter from
//! [SDF](https://datatracker.ietf.org/doc/html/draft-ietf-asdf-sdf-05)
//! (including protocol bindings) to [WoT TD](https://www.w3.org/TR/wot-thing-description/).
//!
//! The converter is both usable as a library that can be built upon in other WoT
//! and SDF related projects as well as a tool for the command line.

use std::{error, result};

pub mod converter;
mod model;
pub mod sdf;
pub mod wot;

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;
