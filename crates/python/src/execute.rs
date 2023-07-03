use crate::PythonLanguage;
use proto_core::{async_trait, get_home_dir, Describable, Executable, Installable, ProtoError};
use std::{
    env,
    path::{Path, PathBuf},
};

// These methods are only used for "is setup" detection,
// and are not actually used for execution. Rely on `~/.cargo/bin` instead.
#[async_trait]
impl Executable<'_> for PythonLanguage {
    async fn find_bin_path(&mut self) -> Result<(), ProtoError> {
        let bin_path = self
            .get_install_dir()?
            .join("install")
            .join("bin")
            .join("python3");

        if bin_path.exists() {
            self.bin_path = Some(bin_path);
        } else {
            return Err(ProtoError::ExecuteMissingBin(self.get_name(), bin_path));
        }

        Ok(())
    }

    fn get_bin_path(&self) -> Result<&Path, ProtoError> {
        match self.bin_path.as_ref() {
            Some(bin) => Ok(bin),
            None => Err(ProtoError::MissingTool(self.get_name())),
        }
    }

    fn get_globals_bin_dir(&self) -> Result<PathBuf, ProtoError> {
        let root = if let Ok(root) = env::var("RYE_HOME") {
            PathBuf::from(root)
        } else {
            get_home_dir()?.join(".rye")
        };

        Ok(root.join("tools"))
    }
}
