//!
//! # Runtime
//! This module provides the complete runtime for interacting with finternet.
//! The runtime exposes 2 APIs:
//! 1. privilaged API: This is used to load drivers that can be used by the process layer to perform operations on the assets.
//! 2. DSL runtime: This allows users to access/execute programss that they have submitted or that
//!    are available in the system. These programs interface with the VAL to perform operations on
//!    the assets.
//!

pub mod driver;
pub mod pal;
pub mod platform;
pub mod privi;
pub mod process;
pub mod val;
pub mod types;
