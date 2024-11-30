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

use super::types as rt;

pub trait CommonVal {
    type Ctx;

    //
    //
    /// Intend to perform an operation on the asset.
    fn intend(&self, ctx: Self::Ctx, ident: rt::RType) -> crate::Result<rt::RType>; // descriptor
    /// This is called when the process is done with the asset.
    fn done(&self, ctx: Self::Ctx, desc: rt::RType) -> crate::Result<()>;

    //
    //
    /// Function to transfer assets from one place to another.
    fn transfer(
        &self,
        ctx: Self::Ctx,
        from: rt::RType,
        to: rt::RType,
        quant: rt::RType,
    ) -> crate::Result<()>;

    //
    //
    /// View the asset.
    fn view(&self, ctx: Self::Ctx, desc: rt::RType) -> crate::Result<rt::RType>; // asset view
}

pub trait PrivilegedVal {
    type Ctx;

    fn load_driver(&self, ctx: Self::Ctx, driver: rt::RType) -> crate::Result<()>;
    fn unload_driver(&self, ctx: Self::Ctx, ident: rt::RType) -> crate::Result<()>;

    fn mount(&self, ctx: Self::Ctx, driver_ident: rt::RType, ident: rt::RType)
        -> crate::Result<()>;

    fn unmount(&self, ctx: Self::Ctx, ident: rt::RType) -> crate::Result<()>;
}
