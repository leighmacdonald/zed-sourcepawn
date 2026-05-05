use std::fs;

use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan, LanguageServerId, Result};

#[derive(Default)]
pub struct SourcepawnLsp {
    cached_binary_path: Option<String>,
}

impl SourcepawnLsp {
    pub const SERVER_ID: &'static str = "sourcepawn-studio";

    pub fn new() -> Self {
        Self::default()
    }

    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("sourcepawn-studio") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "Sarrus1/sourcepawn-studio",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let version = &release.version;
        let asset_name = format!(
            "sourcepawn-studio-{version}-{os}-{arch}.{extension}",
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X8664 => "amd64",
                zed::Architecture::X86 =>
                    return Err(format!("unsupported architecture: {arch:?}")),
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            extension = match platform {
                zed::Os::Windows => "zip",
                _ => "tar.gz",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("sourcepawn-studio-{}", release.version);
        let binary_path = format!("{version_dir}/sourcepawn-studio");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                    _ => zed::DownloadedFileType::GzipTar,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed_extension_api::Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![String::from("-vv")],
            env: worktree.shell_env(),
        })
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        use CompletionKind::*;

        let kind = completion.kind?;

        match kind {
            Class | Enum | Interface | Keyword | Module | Struct => {
                let highlight_name = match completion.kind? {
                    Class | Interface | Enum | Struct => Some("type".to_string()),
                    Keyword => Some("keyword".to_string()),
                    _ => None,
                };

                Some(CodeLabel {
                    code: Default::default(),
                    filter_range: (0..completion.label.len()).into(),
                    spans: vec![CodeLabelSpan::literal(completion.label, highlight_name)],
                })
            }
            EnumMember => {
                let start = "enum Enum { case ";
                let code = format!("{start}{} }}", completion.label);

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        start.len()..start.len() + completion.label.len(),
                    )],
                    filter_range: (0..completion.label.find('(').unwrap_or(completion.label.len()))
                        .into(),
                })
            }
            Function => {
                let func = "func ";
                let mut return_type = String::new();

                if let Some(detail) = completion.detail {
                    if !detail.is_empty() {
                        return_type = format!(" -> {detail}");
                    }
                }

                let before_braces = format!("{func}{}{return_type}", completion.label);
                let code = format!("{before_braces} {{}}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(func.len()..before_braces.len())],
                    filter_range: (0..completion.label.find('(')?).into(),
                })
            }
            TypeParameter => {
                let typealias = "typealias ";
                let code = format!("{typealias}{} = {}", completion.label, completion.detail?);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(typealias.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            Value => {
                let mut r#type = String::new();

                if let Some(detail) = completion.detail {
                    if !detail.is_empty() {
                        r#type = format!(": {detail}");
                    }
                }

                let var = format!("var variable{type} = ");
                let code = format!("{var}{}", completion.label);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(var.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            Variable => {
                let var = "var ";
                let code = format!("{var}{}: {}", completion.label, completion.detail?);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(var.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            _ => None,
        }
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        match symbol.kind {
            SymbolKind::Method | SymbolKind::Function => {
                // Simple label: "func <name>"
                let code = format!("func {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            SymbolKind::Variable | SymbolKind::Constant => {
                // Simple label: "var/let <name>"
                let code = format!("var/let {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            SymbolKind::Class => {
                // Simple label: "class <name>"
                let code = format!("class {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            SymbolKind::Struct => {
                // Simple label: "struct <name>"
                let code = format!("struct {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            SymbolKind::Enum => {
                // Simple label: "enum <name>"
                let code = format!("enum {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            _ => None,
        }
    }
}
