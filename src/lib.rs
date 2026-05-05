use zed_extension_api::{self as zed, LanguageServerId, Result};

struct SourcepawnExtension;

impl zed::Extension for SourcepawnExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: String::from("/projects/sourcepawn-studio/sourcepawn-studio"),
            args: vec![String::from("-vv")],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(SourcepawnExtension);
