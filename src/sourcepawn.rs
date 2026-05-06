mod language_server;

use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId, Result};
use zed_extension_api::{CodeLabel, CodeLabelSpan};

use crate::language_server::SourcepawnLsp;

#[derive(Default)]
struct SourcepawnExtension {
    sourcepawn_lsp: Option<SourcepawnLsp>,
}

impl zed::Extension for SourcepawnExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            SourcepawnLsp::SERVER_ID => {
                let lsp = self.sourcepawn_lsp.get_or_insert_with(SourcepawnLsp::new);
                lsp.language_server_command(language_server_id, worktree)
            }
            _ => Err(format!("Unknown language server: {}", language_server_id)),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let x = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .unwrap();
        let root = worktree.root_path();
        println!("{:?}", root);
        println!("{:?}", &x);

        let opts = x.initialization_options.clone();

        Ok(Some(serde_json::json!(opts)))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
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

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
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
    // fn label_for_completion(
    //     &self,
    //     language_server_id: &LanguageServerId,
    //     completion: Completion,
    // ) -> Option<CodeLabel> {
    //     match language_server_id.as_ref() {
    //         SourcepawnLsp::SERVER_ID => self
    //             .sourcepawn_lsp
    //             .as_ref()?
    //             .label_for_completion(completion),
    //         _ => None,
    //     }
    // }

    // fn label_for_symbol(
    //     &self,
    //     language_server_id: &LanguageServerId,
    //     symbol: Symbol,
    // ) -> Option<CodeLabel> {
    //     match language_server_id.as_ref() {
    //         SourcepawnLsp::SERVER_ID => self.sourcepawn_lsp.as_ref()?.label_for_symbol(symbol),
    //         _ => None,
    //     }
    // }
}

zed::register_extension!(SourcepawnExtension);
