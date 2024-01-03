// mod json;
// mod macros;
// mod mg_compiler;

use anyhow::{Context, Result};
use proc::{generate_single_package_manager_json_struct, generate_single_package_manager_function};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};
use crate::macros::{package_manager_function, package_manager_json};

mod files {
    pub const PACMAN: &str = "db/pacman.json";
    // pub const APT: &str = "db/apt.json";
    // pub const DNF: &str = "db/dnf.json";
    // pub const BREW: &str = "db/brew.json";
    // pub const CHOCO: &str = "db/choco.json";
    // pub const EMERGE: &str = "db/emerge.json";
    // pub const NIX: &str = "db/nix.json";
    // pub const PKG: &str = "db/pkg.json";
    // pub const PKGNG: &str = "db/pkgng.json";
    // pub const PORTAGE: &str = "db/portage.json";
    // pub const SNAP: &str = "db/snap.json";
    // pub const YUM: &str = "db/yum.json";
}

// package_manager_json!(pacman, apt, dnf, brew, choco, emerge, nix, pkg, pkgng, portage, snap, yum);
// package_manager_function!(pacman, apt, dnf, brew, choco, emerge, nix, pkg, pkgng, portage, snap, yum);

package_manager_json!{ pacman }
package_manager_function!{ pacman }