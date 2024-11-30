use core::marker::PhantomData;

mod formatter;

///
/// [`RType`] is a generic type that is used to represent different types on the system.
///
pub struct RType<Format = formatter::Json>(Format::SType, PhantomData<Format>)
where
    Format: formatter::Formatter;
