use zed_extension_api::{self as zed, LanguageServerId, Result};

struct CA65Extension {
    cached_binary_path: Option<String>,
}

impl CA65Extension {
    fn language_server_binary_path(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            return Ok(path.clone());
        }

        if let Some(path) =
            Some("/Users/user/Documents/asm6502-lsp/target/debug/asm6502-lsp".to_string())
        {
            self.cached_binary_path = Some(path.clone());
            return Ok(path.clone());
        }

        Err(
            "asm-lsp binary not found in PATH. Please install with `cargo install asm-lsp` or download from https://github.com/bergercookie/asm-lsp".to_owned()
        )
    }
}

impl zed::Extension for CA65Extension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(CA65Extension);
