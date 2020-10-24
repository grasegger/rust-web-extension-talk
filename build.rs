// use wasm_pack::command::run_wasm_pack as wasm_pack;
use std::fs;
use std::path::{Path, PathBuf};

use wasm_pack::command::build::BuildOptions;
use wasm_pack::command::build::Target;
use wasm_pack::command::build::Build;
use std::process::Command;

use std::fs::OpenOptions;
use std::io::prelude::*;


pub fn main() {
    scripts().unwrap();
    copy_artifacts().unwrap();
    install_yarn();
}

fn scripts () -> Result<(), std::io::Error> {
    let path = Path::new("./scripts");
    for script in fs::read_dir(path)? {
        let script = script?;
        let path = script.path();
        let package_name = path.to_str().unwrap().replace("./scripts/", "");
        build_script(package_name, path);
    }
    
    Ok(())
}

fn build_script(name: String, path: PathBuf){
    let build_opts = BuildOptions{ 
        path: Some(path), 
        disable_dts: true, 
        target: Target::NoModules, 
        out_name: Some(name.clone()),
        out_dir: "../../pkg".to_string(),
        ..Default::default()};

    let mut command = Build::try_from_opts(build_opts).unwrap();
    command.run().unwrap();

    let formated_dest_path = format!("pkg/{}.js", name);
    let js_dest_path = Path::new(&formated_dest_path);

    let mut js_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(js_dest_path)
        .unwrap();

    writeln!(js_file, "wasm_bindgen(browser.runtime.getURL('{}_bg.wasm'));", name).unwrap();
//    writeln!(js_file, format!("wasm_bindgen({}_bg.wasm);", name));
}

fn copy_artifacts () -> Result<(), std::io::Error>  {
    let path = Path::new("./artifacts");
    for artifact in fs::read_dir(path)? {
        let artifact = artifact?;
        let path = artifact.path();
        let target = String::from(format!("./pkg/{}", path.to_str().unwrap().replace("./artifacts/", "")));
        println!("{}", target);
        println!("{:?}", path);
        fs::copy(path, target)?;
    }

    Ok(())
}

fn install_yarn()  {
    let mut command = Command::new("yarn");
    command
        .current_dir("./pkg")
        .output()
        .unwrap();
}