use crate::tools::{create_tool, ToolType};
use proto_core::{detect_version, Manifest};
use starbase::SystemResult;

pub async fn bin(
    tool_type: ToolType,
    forced_version: Option<String>,
    use_shim: bool,
) -> SystemResult {
    let mut tool = create_tool(&tool_type)?;
    let manifest = Manifest::load(tool.get_manifest_path())?;
    let version = detect_version(&tool, &manifest, forced_version).await?;

    tool.resolve_version(&version).await?;
    tool.find_bin_path().await?;

    if use_shim {
        tool.create_shims().await?;

        if let Some(shim_path) = tool.get_shim_path() {
            println!("{}", shim_path.to_string_lossy());

            return Ok(());
        }
    }

    println!("{}", tool.get_bin_path()?.to_string_lossy());

    Ok(())
}
