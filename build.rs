use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap();

    // src/shaders -> destino: target/{debug|release}/shaders
    let src_dir = manifest_dir.join("src/shaders");
    let dest_dir = manifest_dir.join("target").join(&profile).join("shaders");
    fs::create_dir_all(&dest_dir).unwrap();

    for entry in fs::read_dir(src_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("glsl") {
            fs::copy(&path, dest_dir.join(path.file_name().unwrap())).unwrap();
        }
    }

    // faz Cargo rerodar o script se algum shader mudar
    println!("cargo:rerun-if-changed=src/shaders");
}
