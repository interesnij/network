pub mod pages;
pub mod progs;
pub mod owner_progs;
pub mod manager_progs;
pub mod settings_progs;

pub use self::{
    pages::*,
    progs::*,
    owner_progs::*,
    manager_progs::*,
    settings_progs::*,
};
