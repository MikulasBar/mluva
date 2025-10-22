pub mod init;
pub mod run;
pub mod uninit;
pub mod build;

use clap::Subcommand;


#[derive(Subcommand)]
pub enum Commands {
    Init,
    Run,
    Uninit,
    Build,
}

const META_DIR: &str = ".mluva";
const MODULES_META_FILE: &str = "modules.yaml";
const CONFIG_FILE: &str = "mluva.yaml";
const ROOT_MODULE_DEFAULT_CONTENT: &str = r#"# This is the root module of your Mluva project.
# You can change the name of this file in the 'mluva.yaml' configuration file.
# Happy coding!

Float main() {
    return 0.0
}
"#;