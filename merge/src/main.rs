mod json;
mod macros;
// mod mg_compiler;

use anyhow::{Context, Result};
use proc::generate_single_package_manager_json_struct;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

package_manager_json! { pacman }

fn main() -> Result<()> {
    let file_json = File::open("db/pacman.json").context("Couldn't find file db/pacman.json")?;
    let mut buf_reader = BufReader::new(file_json);
    let mut json_buf = String::with_capacity(500);

    buf_reader
        .read_to_string(&mut json_buf)
        .context("Couldn't read buffer to String")?;

    // Parse the string of data into a MIR Representation
    let pacman_json: PacmanJson = serde_json::from_str(&json_buf)?;

    // pacman_json.pacman.iter_mut().map(|(i, j)| );
    Ok(())
}
