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
    let system =
        unsafe { StabbyLibrary::get_stabbied::<extern fn() -> BoxedSystem>(&lib, b"get_system") }
            .unwrap()();
    //let a = stable_fn();

    //let a = (a.get_shared)();
    //a.get_system().run();
    //let a = a.get_data();
    /*println!(
        "it worknigngggggggggggggggggggggggggggggg!@!!!!!!!!!!!!!!!##2323323!3 ggs, {:?}",
        a
    );*/
}
