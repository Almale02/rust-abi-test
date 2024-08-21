use interface::prelude::*;

#[stabby::export]
#[stabby]
pub extern fn get_plugin() -> Plugin {
    Plugin { get_shared }
}
pub struct GetSharedImpl;

impl SharedTrait for GetSharedImpl {
    extern fn get_data(&self) -> u32 {
        1
    }
}
pub extern fn get_shared() -> GetSharedOutType {
    RBox::new(GetSharedImpl).into()
}
