use zed_extension_api as zed;

struct CircomExtension {}

fn locate_lsp(
    language_server_id: &zed::LanguageServerId,
    worktree: &zed::Worktree,
) -> Result<String, String> {
    match worktree.which("circom-lsp") {
        Some(lsp_path) => Ok(lsp_path),
        None => {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::CheckingForUpdate,
            );

            let circom_lsp_repo = "rubydusa/circom-lsp";
            let release = zed::latest_github_release(
                circom_lsp_repo,
                zed::GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )?;
            let platform = match zed::current_platform() {
                (zed::Os::Linux, _) => "linux",
                (zed::Os::Mac, _) => "macos",
                (zed::Os::Windows, _) => "windows",
            };
            let asset = release
                .assets
                .iter()
                .find(|x| x.name.contains(platform))
                .ok_or("No lsp found for your platform")?;

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let bin_path = "circom-lsp".to_string();
            zed::download_file(
                &asset.download_url,
                &bin_path,
                zed::DownloadedFileType::Uncompressed,
            )?;

            zed::make_file_executable(&bin_path)?;

            Ok(bin_path)
        }
    }
}

impl zed::Extension for CircomExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        CircomExtension {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_path = locate_lsp(language_server_id, worktree)?;

        Ok(zed::process::Command::new(lsp_path).envs(worktree.shell_env()))
    }
}

zed::register_extension!(CircomExtension);
