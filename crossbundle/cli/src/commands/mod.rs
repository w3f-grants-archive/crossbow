pub mod build;
pub mod install;
pub mod new;
pub mod run;

use crate::error::Result;
use clap::Parser;
use crossbundle_tools::types::Config;

#[derive(Parser, Clone, Debug)]
pub enum Commands {
    /// Starts the process of building/packaging/signing of the rust crate
    #[clap(subcommand)]
    Build(build::BuildCommand),
    /// Executes `build` command and then deploy and launches the application on the
    /// device/emulator
    #[clap(subcommand)]
    Run(run::RunCommand),
    /// Creates a new Cargo package in the given directory. Project will be ready to build
    /// with `crossbundle`
    New(new::NewCommand),
    /// Installs bundletool and Android Studio's sdkmanager
    Install(install::InstallCommand),
}

impl Commands {
    pub fn handle_command(&self, config: &Config) -> Result<()> {
        match self {
            Commands::Build(cmd) => cmd.handle_command(config),
            Commands::Run(cmd) => cmd.handle_command(config),
            Commands::New(cmd) => cmd.handle_command(config),
            Commands::Install(cmd) => cmd.handle_command(config),
        }
    }
}
