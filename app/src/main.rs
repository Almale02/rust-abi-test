use interface::{prelude::*, SharedTraitDyn};
use stabby::libloading::StabbyLibrary;
use std::env::current_dir;

fn main() {
    let mut plugin_path: String = format!(
        "{}/{}",
        current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
        "plugins"
    );
    let path = format!("{}/{}", plugin_path, "plugin_1.dll");
    println!("the path was: {:?}", path);
    println!("current working dir: {:?}", current_dir().unwrap());

    let lib = unsafe { libloading::Library::new(path).unwrap() };

    let plugin =
        unsafe { StabbyLibrary::get_stabbied::<extern fn() -> Plugin>(&lib, b"get_plugin") }
            .unwrap()();
    let mut system = (plugin.get_system)();
    system.run();
    system.run();
    system.run();
    system.run();
}
