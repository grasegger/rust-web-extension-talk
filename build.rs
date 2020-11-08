// use wasm_pack::command::run_wasm_pack as wasm_pack;
use std::path::{Path, PathBuf};
use std::{
    error::Error,
    fs::{self, File},
};

use std::process::Command;
use wasm_pack::command::build::Build;
use wasm_pack::command::build::BuildOptions;
use wasm_pack::command::build::Target;

use std::fs::OpenOptions;
use std::io::prelude::*;

use json::{object, stringify_pretty, JsonValue};
use toml::Value;

pub fn main() {
    create_manifest().expect("There was an error creating the manifest file.");
    copy_artifacts().expect("I was unable to copy the sources from the artifacts folder.");
    install_yarn().expect("I was not able to execute yarn in the pkg folder correctly.");
}

fn create_manifest() -> Result<(), Box<dyn Error>> {
    let cargo_file_content = read_cargo_file("Cargo.toml".to_string())?;
    let parsed_cargo_file = cargo_file_content.parse::<Value>()?;
    let gecko_id = object!{
        gecko: object!{ 
            id: parsed_cargo_file["package"]["metadata"]["webextension"]["gecko_id"].as_str() 
        }
    };
    let mut manifest_json = object! {
        manifest_version: 2,
        version: parsed_cargo_file["package"]["version"].as_str(),
        description: parsed_cargo_file["package"]["description"].as_str(),
        author: parsed_cargo_file["package"]["authors"][0].as_str(),
        name: parsed_cargo_file["package"]["metadata"]["webextension"]["extension_name"].as_str(),
        content_scripts: Vec::<JsonValue>::new(),
        web_accessible_resources: Vec::<JsonValue>::new(),
        permissions: Vec::<JsonValue>::new(),
        applications: gecko_id.clone(),
        browser_specific_settings: gecko_id,
    };

    let artifacts = Path::new("./artifacts");
    for script in fs::read_dir(artifacts)? {
        let script = script?;
        let path = script.path().to_str().unwrap().replace("./artifacts/", "");
        manifest_json["web_accessible_resources"].push(path)?;
    }

    for permission in parsed_cargo_file["package"]["metadata"]["webextension"]["permissions"].as_array().unwrap() {
        manifest_json["permissions"].push(permission.as_str())?;
    }

    for content_script in parsed_cargo_file["package"]["metadata"]["webextension"]
        ["content_scripts"]
        .as_array()
        .unwrap()
    {
        let mut script = object! {
            matches: Vec::<JsonValue>::new(),
            js: Vec::<JsonValue>::new(),
            css: Vec::<JsonValue>::new(),
        };

        for site_match in content_script["matches"].as_array().unwrap() {
            script["matches"].push(site_match.as_str())?;
        }

        for js in content_script["js"].as_array().unwrap() {
            let source = js.as_str().unwrap();
            let extension = source.split(".").last().unwrap();

            if extension == "toml" {
                let package_name = get_package_name_from_path(source.into()).unwrap();
                let source_folder = source.split("/Cargo.toml").next().unwrap();
                build_script(package_name.clone(), Path::new(source_folder).into());

                script["js"].push(format!("{}.js", &package_name))?;
                manifest_json["web_accessible_resources"]
                    .push(format!("{}_bg.wasm", &package_name))?;
            } else {
                script["js"].push(source)?;
            }
        }

        manifest_json["content_scripts"].push(script)?;
    }

    let pkg_path = Path::new("pkg");

    if !pkg_path.exists() {
        fs::create_dir(pkg_path)?;
    }

    let manifest_path = Path::new("pkg/manifest.json");
    write_json(manifest_json, manifest_path)?;
    Ok(())
}

fn get_package_name_from_path(path: String) -> Result<String, Box<dyn Error>> {
    let cargo_contents_string = read_cargo_file(path)?;
    let cargo_contents_toml = cargo_contents_string.parse::<Value>()?;
    Ok(cargo_contents_toml["package"]["name"]
        .as_str()
        .unwrap()
        .to_string())
}

fn read_cargo_file(path: String) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_json(json: JsonValue, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(stringify_pretty(json, 2).as_bytes())?;
    Ok(())
}

fn build_script(name: String, path: PathBuf) {
    let build_opts = BuildOptions {
        path: Some(path),
        disable_dts: true,
        target: Target::NoModules,
        out_name: Some(name.clone()),
        out_dir: "../../pkg".to_string(),
        ..Default::default()
    };

    let mut command = Build::try_from_opts(build_opts).unwrap();
    command.run().unwrap();

    let formated_dest_path = format!("pkg/{}.js", name);
    let js_dest_path = Path::new(&formated_dest_path);

    let mut old_js_file_content = fs::read_to_string(js_dest_path).unwrap();

    old_js_file_content += &format!("wasm_bindgen(browser.runtime.getURL('{}_bg.wasm'));", name);


    let mut js_file = OpenOptions::new()
        .write(true)
        .append(false)
        .open(js_dest_path)
        .unwrap();

    write!(
        js_file,
        "(function () {{ {} }})()",
        old_js_file_content
    )
    .unwrap();
}

fn copy_artifacts() -> Result<(), std::io::Error> {
    let path = Path::new("./artifacts");
    for artifact in fs::read_dir(path)? {
        let artifact = artifact?;
        let path = artifact.path();
        let target = String::from(format!(
            "./pkg/{}",
            path.to_str().unwrap().replace("./artifacts/", "")
        ));
        println!("{}", target);
        println!("{:?}", path);
        fs::copy(path, target)?;
    }

    Ok(())
}

fn install_yarn() -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("yarn");
    command.current_dir("./pkg").output().expect("Error while installing the yarn dependencies.");
    Ok(())
}
