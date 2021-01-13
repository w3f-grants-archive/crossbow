use crate::commands::build::{android::AndroidBuildCommand, BuildContext};
use crate::*;
use clap::Clap;
use creator_tools::*;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub struct AndroidRunCommand {
    #[clap(flatten)]
    pub build_command: AndroidBuildCommand,
}

impl AndroidRunCommand {
    pub fn run(&self, current_dir: PathBuf) -> Result<()> {
        let build_context =
            BuildContext::init(&current_dir, self.build_command.shared.target_dir.clone())?;
        let (package_name, sdk, apk_path) = self.build_command.execute(&build_context)?;
        info!("Starting run process");
        install_apk(&sdk, &apk_path)?;
        start_apk(&sdk, &package_name)?;
        info!("Run finished successfully");
        Ok(())
    }
}
