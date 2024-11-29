//!
//! # VAL (Virtual Asset Layer)
//!
//! The Virtual Asset Layer (VAL) is an abstraction over the [`resolver`] and [`driver`].
//!
//! This is the layer that is to be implemented by the drivers that are being onboarded into the
//! system. This is exposed to the process layer to perform operations on the assets.
//! This interface consists of 2 contracts:
//! 1. Privileged: This are functions that are implemented on the [`resolver`] and [`driver`] and
//!    are never implemented insider the driver. These are privileged functions responsible for
//!    interacting with the drivers.
//! 2. Common: These are functions that are implemented by the driver and are used by the
//!    processes.
//!
