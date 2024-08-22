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
extern fn get_shared() -> GetSharedOutType {
    RBox::new(GetSharedImpl).into()
}
fn system(x: u32, y: u32) {
    println!("rana from u32 system lib, x: {}, y: {}", x, y);
}
#[stabby::export]
#[stabby]
extern fn get_system() -> BoxedSystem {
    dbg!("b");
    RBox::new(system).system()
}
