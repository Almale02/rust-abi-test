use std::path::Path;

fn main() {
    let path_to_dll = "../target/debug/plugin_1.dll";

    let target = format!(
        "{}/plugins/plugin_1.dll",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let target = Path::new(target.as_str());

    if target.exists() {
        std::fs::remove_file(&target).expect("Failed to remove the existing DLL");
    }
    std::fs::copy(&path_to_dll, &target).expect(&format!(
        "failed to copy the dll to the directory: {:?}",
        target
    ));

    println!("cargo:rerun-if-changed={}", path_to_dll);
}
