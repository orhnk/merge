use clap::{Parser, ValueEnum};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
pub struct MergeArgs {
    /// Package Manager Backend to get used
    pub package_manager: MergePackageManager,

    // NOTE: maybe a string here could be enough
    /// Actions to perform on packages
    #[arg(allow_hyphen_values = true)]
    pub action: String,

    // #[arg(last = true)]
    pub packages: Vec<String>,
    // #[arg(last(true))]
    // pub raw_args: Vec<String>,
}

// TODO: move this to a separate file
#[derive(ValueEnum, Debug, Clone, Display, PartialEq, Eq, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum MergePackageManager {
    Pacman,
    Apt,
    Nix,
    Yum,
    Brew,
    Choco,
    Scoop,
    Dnf,
    Zypper,
    Apk,
    Emerge,
}

//'''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''''
// | `APT`            | `apt install <paket>`    | `apt upgrade <paket>`      | `apt search <paket>`      | `apt remove <paket>`       |
// | `Pacman`         | `pacman -S <paket>`      | `pacman -S <paket>`        | `pacman -Ss <paket>`      | `pacman -Rsc <paket>`      |
// | `Nix`            | `nix-env -i <paket>`     | `nix search <paket>`       | `nix-env -u <paket>`      | `nix -e <paket>`           |
// | `Homebrew`       | `brew install <paket>`   | `brew upgrade <paket>`     | `brew search <paket>`     | `brew uninstall <paket>`   |
// | `Chocolatey`     | `choco install <paket>`  | `choco upgrade <paket>`    | `choco search <paket>`    | `choco uninstall <paket>`  |
// | `Scoop`          | `scoop install <paket>`  | `scoop update <paket>`     | `scoop search <paket>`    | `scoop uninstall <paket>`  |
// | `Yum`            | `yum install <paket>`    | `yum update <paket>`       | `yum search <paket>`      | `yum remove <paket>`       |
// | `Dnf`            | `dnf install <paket>`    | `dnf update <paket>`       | `dnf search <paket>`      | `dnf remove <paket>`       |
// | `Zypper`         | `zypper install <paket>` | `zypper update <paket>`    | `zypper search <paket>`   | `zypper remove <paket>`    |
// | `APK`            | `apk add <paket>`        | `apk upgrade <paket>`      | `apk search <paket>`      | `apk del <paket>`          |
// | `Xbps`           | `xbps-install <paket>`   | `xbps-install -Su <paket>` | `xbps-query -Rs <paket>`  | `xbps-remove <paket>`      |
// | `RPM`            | `rpm -i <paket>`         | `rpm -U <paket>`           | `rpm -qf <paket>`         | `rpm -e <paket>`           |
// | `Portage`        | `emerge <paket>`         | `emerge --update <paket>`  | `emerge --search <paket>` | `emerge --unmerge <paket>` |
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
