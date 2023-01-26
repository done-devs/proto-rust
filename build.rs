use std::{env, path::PathBuf};

use prost_wkt_build::{FileDescriptorSet, Message};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(
            ".",
            "#[derive(serde::Serialize,serde::Deserialize)] #[serde(rename_all = \"camelCase\")]",
        )
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .file_descriptor_set_path(&descriptor_file)
        .out_dir("src/")
        .compile_protos(&["proto/provider.proto"], &["proto/"])
        .unwrap();

    let descriptor_bytes = std::fs::read(descriptor_file).unwrap();

    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    prost_wkt_build::add_serde(out, descriptor);
    Ok(())
}
