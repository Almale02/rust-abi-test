#![feature(freeze)]

use std::marker::PhantomData;

use stabby::boxed::Box as RBox;
use stabby::{dynptr, stabby};

pub mod prelude {
    pub use crate::BoxedSystem;
    pub use crate::GetSharedOutType;
    pub use crate::IntoSystem;
    pub use crate::Plugin;
    pub use crate::SharedTrait;
    pub use crate::SharedTraitDyn;
    pub use crate::SharedTraitDynMut;
    pub use crate::System;
    pub use crate::SystemDyn;
    pub use crate::SystemDynMut;
    pub use stabby::boxed::Box as RBox;
    pub use stabby::closure as rclosure;
    pub use stabby::dynptr;
    pub use stabby::stabby;
    pub use stabby::string::String as RString;
    pub use stabby::Dyn;
}

#[stabby::stabby]
pub trait SharedTrait {
    extern fn get_data(&self) -> u32;
}

#[stabby::stabby]
pub trait System {
    extern fn run(&mut self);
}
pub trait IntoSystem<Marker> {
    extern fn system(self) -> BoxedSystem;
}

pub type GetSharedOutType = dynptr!(RBox<dyn SharedTrait>);
pub type BoxedSystem = dynptr!(RBox<dyn System>);

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

macro_rules! implement_into_system {
    ($($param:ident),*) => {
        impl<Func, $($param),*> IntoSystem<fn($($param), *)>
            for Func
        where
            $($param: Default + 'static),*,
            Func: Fn($($param), *) + 'static,
        {
            extern fn system(self) -> BoxedSystem {
                RBox::new(FunctionSystem::new(self)).into()
            }
        }
    };
}

#[stabby]
pub struct FunctionSystem<Func: 'static, Marker> {
    pub function: Func,
    _p: PhantomData<Marker>,
}

impl<Func: 'static, Marker> FunctionSystem<Func, Marker> {
    pub fn new(function: Func) -> Self {
        Self {
            function,
            _p: PhantomData,
        }
    }
}

macro_rules! impl_schedule_runnable {
    ($($param:ident),*) => {
        //impl<$($param),*, Func> ScheduleRunnable for (&'static str, Func, Vec<Box<dyn SystemParalellFilter>>, std::marker::PhantomData<fn($($param),*)>  )
        impl<$($param),*, Func> System for FunctionSystem<Func, fn($($param),*)>
        where
            $($param: Default + 'static),*,
            Func: Fn($($param),*)
        {
            extern fn run(&mut self) {
                (self.function)($($param::default()),*)
            }
        }
    };
}
impl_schedule_runnable!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_schedule_runnable!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_schedule_runnable!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_schedule_runnable!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_schedule_runnable!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_schedule_runnable!(A, B, C, D, E, F, G, H, I, J, K);
impl_schedule_runnable!(A, B, C, D, E, F, G, H, I, J);
impl_schedule_runnable!(A, B, C, D, E, F, G, H, I);
impl_schedule_runnable!(A, B, C, D, E, F, G, H);
impl_schedule_runnable!(A, B, C, D, E, F, G);
impl_schedule_runnable!(A, B, C, D, E, F);
impl_schedule_runnable!(A, B, C, D, E);
impl_schedule_runnable!(A, B, C, D);
impl_schedule_runnable!(A, B, C);
impl_schedule_runnable!(A, B);
impl_schedule_runnable!(A);

implement_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
implement_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
implement_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
implement_into_system!(A, B, C, D, E, F, G, H, I, J, K, L, M);
implement_into_system!(A, B, C, D, E, F, G, H, I, J, K, L);
implement_into_system!(A, B, C, D, E, F, G, H, I, J, K);
implement_into_system!(A, B, C, D, E, F, G, H, I, J);
implement_into_system!(A, B, C, D, E, F, G, H, I);
implement_into_system!(A, B, C, D, E, F, G, H);
implement_into_system!(A, B, C, D, E, F, G);
implement_into_system!(A, B, C, D, E, F);
implement_into_system!(A, B, C, D, E);
implement_into_system!(A, B, C, D);
implement_into_system!(A, B, C);
implement_into_system!(A, B);
implement_into_system!(A);
