use std::{env, fs, path::PathBuf};

pub fn load_shader(name: &str) -> String {
    ///Usa relativo ao executável
    let exe_dir: PathBuf = env::current_exe()
        .expect("não achei o próprio exe")
        .parent() // tira o nome do arquivo
        .unwrap()
        .to_path_buf();

    // shaders/vertex.glsl ao lado do exe
    let shader_path = exe_dir.join("shaders").join(name);

    println!("vai buscar shader em: {:?}", shader_path.as_os_str());
    return fs::read_to_string(&shader_path).expect(&format!("shader não encontrado {:?}", shader_path));
}
