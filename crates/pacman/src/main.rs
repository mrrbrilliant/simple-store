mod install;
mod list;
mod remove;
mod update;

use list::{are_installed, is_installed};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let all = are_installed(&["atoms"]);

    // if let Some(pkgs) = all {
    //     pkgs.iter().for_each(|pkg| println!("\n{}\n", &pkg));
    // } else {
    //     println!("Packages are not installed");
    // }

    // let atom = is_installed("atoms");

    // if let Some(pkg) = atom {
    //     println!("\n{}\n", pkg);
    // } else {
    //     println!("Package is not installed");
    // }

    // remove::remove_orphans();

    if let Ok(()) = install::smart_install(&["cowsay"], true) {
        println!("Installation succeeded");
    }

    if let Ok(()) = remove::smart_remove(&vec!["cowsay"]) {
        println!("Removing succeeded")
    }

    if let Ok(()) = update::smart_update(false) {
        println!("Updating succeeded")
    }

    Ok(())
}
