use std::{path::PathBuf, env, fs::{self, OpenOptions}, io::Write};

use toml::Value;

const DATA_FILE: &str = "../build.toml";

const MY_TOML: &str = include_str!("./Cargo.toml");

fn main() {
    println!("cargo:rerun-if-changed=src/myuifile.fl");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let binding = out_path.join("defaults.rs");
    let out = binding.to_str().unwrap();

    let data: Value = toml::from_str( &fs::read_to_string(DATA_FILE).unwrap() ).unwrap();

    let mut to_write = Vec::new();

    let protocol_details = &data["protocol"];
    let protcol_human_version = protocol_details["version"].as_integer().unwrap();

    let defaults = &data["defaults"];
    let port = format!("pub const PORT: i64 = {};", defaults["port"].as_integer().unwrap());

    to_write.push(port.clone());

    // Protocol Version
    let protocol_version: Value = toml::from_str( MY_TOML ).unwrap();
    let protcol_version = protocol_version["package"]["version"].as_str().unwrap();

    let protcol_version = format!("{protcol_human_version}{protcol_version}");
    let protcol_version: u64 = protcol_version.replace(".", "").parse().unwrap();

    let protcol_version = format!("pub const PROTOCOL_VERSION: u64 = {};", protcol_version);

    to_write.push(protcol_version.clone());
    // End Protocol Version

    // Client Y Sync
    let data: Value = toml::from_str( &fs::read_to_string(DATA_FILE).unwrap() ).unwrap();

    let data = &data["sync"];
    let protcol_human_version = data["client_y"].as_integer().unwrap();

    let port = format!("pub const CLIENT_Y_SYNC: i64 = {};", protcol_human_version);

    to_write.push(port.clone());
    // End Client Y Sync

    let mut file = OpenOptions::new().write(true).create(true).open(out).unwrap();

    let mut buffer = String::new();
    for line in to_write { buffer += &format!("{line}\n"); }

    file.write(buffer.as_bytes()).unwrap();

    // Complie time validation - Client/Server must be the same version!
    client_server_validation();
}

fn client_server_validation() {
    // package.version

    let server = "../LostServer/Cargo.toml";
    let client = "../LostClient/Cargo.toml";

    let server: Value = toml::from_str( &fs::read_to_string(server).unwrap() ).unwrap();
    let client: Value = toml::from_str( &fs::read_to_string(client).unwrap() ).unwrap();

    let server = server["package"]["version"].as_str().unwrap();
    let client = client["package"]["version"].as_str().unwrap();

    if server == client { return }

    panic!("Incorrect Client/Server versions!")
}