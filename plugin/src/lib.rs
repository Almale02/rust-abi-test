static mut GLOBAL: i32 = 0;

use interface::prelude::*;

#[stabby::export]
#[stabby]
pub extern fn get_plugin() -> Plugin {
    Plugin {
        get_shared,
        get_system,
    }
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
    let global = unsafe { &mut GLOBAL };

    *global += 1;
    println!(
        "ran from u32 system lib, x: {}, y: {}, global: {}",
        x, y, global
    );
}
#[stabby]
extern fn get_system() -> BoxedSystem {
    RBox::new(system).system()
}
