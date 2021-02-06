mod generator;
mod items;
mod producers;
mod recipes;
mod schenum_map;
mod technologies;

use crate::generator::AllDSPInfo;

use anyhow::{anyhow, bail};
use schemars::schema_for;

use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};

fn main() -> anyhow::Result<()> {
    if !cfg!(debug_assertions) {
        bail!("You should only run this in debug mode! This way the program knows where to write the data.")
    }

    println!("Generating DSP info json...");
    let info = AllDSPInfo::generate();
    let info = serde_json::to_string_pretty(&info)?;
    let output_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../site/src/dsp.json");
    println!("Planning to write {} bytes to {}", info.len(), &output_path);
    fs::write(output_path, info)?;

    println!("Generating schema...");
    let schema = schema_for!(AllDSPInfo);

    let schema_string = serde_json::to_string(&schema)?;
    println!("Starting json2ts...");
    let mut child = Command::new(if cfg!(target_os = "windows") {
        "json2ts.cmd"
    } else {
        "json2ts"
    })
    .stdin(Stdio::piped()) // We pipe it in in a moment...
    .stdout(Stdio::piped())
    .spawn()?;
    {
        // Not sure why this is in its own block but that's what the example does
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("could not open stdin"))?;
        stdin.write_all(schema_string.as_bytes())?;
    }
    let output = child.wait_with_output()?;
    if !output.status.success() {
        bail!(
            "Error detected from json2ts (code {}), aborting!",
            output.status
        );
    }

    let output_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../site/src/dsp.d.ts");
    println!(
        "Planning to write {} bytes to {}",
        schema_string.len(),
        &output_path
    );

    fs::write(output_path, output.stdout)?;

    Ok(())
}
