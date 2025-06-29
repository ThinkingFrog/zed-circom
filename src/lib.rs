use zed_extension_api as zed;

struct CircomExtension {
    bin_name: String,
}

impl CircomExtension {
    fn locate_lsp(
        &self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String, String> {
        if let Some(lsp_path) = worktree.which("circom-lsp") {
            return Ok(lsp_path);
        }

        if std::fs::exists(&self.bin_name).unwrap_or(false) {
            return Ok(self.bin_name.clone());
        }

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

        zed::download_file(
            &asset.download_url,
            &self.bin_name,
            zed::DownloadedFileType::Uncompressed,
        )?;

        zed::make_file_executable(&self.bin_name)?;

        Ok(self.bin_name.clone())
    }
}

impl zed::Extension for CircomExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        CircomExtension {
            bin_name: "circom-lsp".to_string(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_path = self.locate_lsp(language_server_id, worktree)?;

        Ok(zed::process::Command::new(lsp_path).envs(worktree.shell_env()))
    }
}

zed::register_extension!(CircomExtension);
