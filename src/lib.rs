use core::panic;

use zed_extension_api as zed;

struct CircomExtension {}

fn locate_cargo(worktree: &zed::Worktree) -> String {
    match worktree.which("cargo") {
        Some(cargo_path) => cargo_path,
        None => {
            panic!("Rust toolchain required to install circom lsp");
        }
    }
}

fn locate_lsp(language_server_id: &zed::LanguageServerId, worktree: &zed::Worktree) -> String {
    match worktree.which("circom-lsp") {
        Some(lsp_path) => lsp_path,
        None => {
            let cargo_path = locate_cargo(worktree);

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::process::Command {
                command: cargo_path,
                args: vec!["install".to_string(), "circom-lsp".to_string()],
                env: worktree.shell_env(),
            };

            match worktree.which("circom-lsp") {
                Some(lsp_path) => lsp_path,
                None => panic!("Failed to download circom-lsp"),
            }
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
        let lsp_path = locate_lsp(language_server_id, worktree);

        Ok(zed::process::Command {
            command: lsp_path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(CircomExtension);
