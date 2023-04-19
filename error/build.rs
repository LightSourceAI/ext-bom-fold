use std::io::Result;

fn main() -> Result<()> {
    let mut builder = prost_build::Config::new();
    builder.type_attribute(
        "lightsource.error.DebugInfo",
        "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
    );
    builder.compile_protos(&["proto/error_details.proto"], &["proto/"])?;
    Ok(())
}
