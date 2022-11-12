pub use lib::Cli;

mod info;
#[allow(clippy::module_inception)]
mod lib;
mod new;
mod pack;

mod prelude {
    pub use super::info::get_project_name;
    pub use super::lib::read_relative_json;
    pub use super::lib::write_relative_json;
}
