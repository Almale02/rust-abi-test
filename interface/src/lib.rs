#![feature(freeze)]

use stabby::boxed::Box as RBox;
use stabby::string::String as RString;
use stabby::{dynptr, stabby, Dyn};

pub mod prelude {
    pub use crate::Shared;
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

#[stabby::stabby]
pub struct Shared {
    pub number: RString,
}
impl SharedTrait for Shared {
    extern "C" fn get_data(&self) -> u32 {
        256
    }
}

#[stabby]
pub fn get_shared() -> dynptr!(RBox<dyn SharedTrait>) {
    RBox::new(Shared {
        number: "value from plugin".into(),
    })
    .into()
}
