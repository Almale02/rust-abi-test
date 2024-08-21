#![feature(freeze)]

use stabby::boxed::Box as RBox;
use stabby::string::String as RString;
use stabby::{dynptr, stabby};

pub mod prelude {
    pub use crate::GetSharedOutType;
    pub use crate::Plugin;
    pub use crate::SharedTrait;
    pub use stabby::boxed::Box as RBox;
    pub use stabby::closure as rclosure;
    pub use stabby::dynptr;
    pub use stabby::stabby;
    pub use stabby::string::String as RString;
    pub use stabby::Dyn;
}

#[stabby::stabby]
#[rustfmt::skip]
pub trait SharedTrait {
    extern fn get_data(&self) -> u32;
}

pub type GetSharedOutType = dynptr!(RBox<dyn SharedTrait>);

/// In your plugin inplementation you have to create an extern function call `get_plugin` which returns the `Plugin` instance
///
/// # Implementation
/// ```
/// pub extern fn get_plugin() -> Plugin;
///
/// ```
//#[allow(clippy::type_complexity)]
#[stabby]
pub struct Plugin {
    pub get_shared: extern fn() -> GetSharedOutType,
}
