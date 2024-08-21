use interface::{prelude::*, SharedTraitDyn};
use rclosure::Call0;
use stabby::libloading::StabbyLibrary;
use std::{env::current_dir, fs::File};

fn main() {
    let mut plugin_path = format!(
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

    let stable_fn =
        unsafe { StabbyLibrary::get_stabbied::<extern fn() -> Plugin>(&lib, b"get_plugin") }
            .unwrap();
    let a = stable_fn();

    let a = (a.get_shared)();
    let a = a.get_data();
    println!(
        "it worknigngggggggggggggggggggggggggggggg!@!!!!!!!!!!!!!!!##2323323!3 ggs, {:?}",
        a
    );
}
