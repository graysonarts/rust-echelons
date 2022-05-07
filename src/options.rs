use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "echelons",
    about = "Creating Directory Hierarchies from a project template",
    rename_all = "kebab-case"
)]
pub(crate) struct Options {
    #[structopt(flatten)]
    pub(crate) verbose: structopt_flags::Verbose,

    #[structopt(short, long, parse(from_os_str))]
    pub(crate) config: Option<PathBuf>,

    /// The directory where you want to create the project
    #[structopt(parse(from_os_str))]
    pub(crate) root_directory: PathBuf,

    /// The name of the project if specified, this directory will be created under the root directory
    #[structopt()]
    pub(crate) project_name: Option<String>,
}

fn root_directory(root: &Path, filename: &str) -> Option<PathBuf> {
    Some(root.join(filename))
}

fn current_directory(filename: &str) -> Option<PathBuf> {
    Some(
        std::env::current_dir()
            .expect("Unable to get current working directory")
            .join(filename),
    )
}

fn home_directory(filename: &str) -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(filename))
}

fn config_directory(sub_path: PathBuf) -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join(sub_path))
}

pub(crate) fn find_config(root: &Path) -> Option<PathBuf> {
    let search_paths = vec![
        root_directory(root, "echelons.toml"),
        current_directory("echelons.toml"),
        home_directory(".echelons.toml"),
        config_directory(Path::new("echelons").join("config.toml")),
    ];

    search_paths
        .iter()
        .find(|x| if let Some(p) = x { p.exists() } else { false })?
        .to_owned()
}
