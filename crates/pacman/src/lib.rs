mod install;
mod list;
mod remove;
mod update;

pub use install::*;
pub use list::*;
pub use remove::*;
pub use update::*;

#[cfg(test)]
mod tests {
    use super::*;
    // installer
    #[test]
    fn install_new_package() {
        assert_eq!(smart_install(&vec!["cowsay"], false).is_ok(), true);
    }
    // remover
    #[test]
    fn remove_package() {
        assert_eq!(smart_remove(&vec!["cowsay"]).is_ok(), true);
    }
    // updater
    #[test]
    fn update() {
        assert_eq!(smart_update(false).is_ok(), true);
    }
}
