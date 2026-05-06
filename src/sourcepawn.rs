mod language_server;

use serde::Serialize;
use std::path::PathBuf;
use zed_extension_api::lsp::{Completion, Symbol};
use zed_extension_api::CodeLabel;
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId, Result};

use crate::language_server::SourcepawnLsp;

#[derive(Serialize, Default)]
#[allow(non_snake_case)]
struct Config {
    cachePriming_enable: bool,
    cachePriming_numThreads: u8,
    compiler_arguments: Vec<String>,
    compiler_path: Option<String>,
    compiler_onSave: bool,
    eventsGameName: Option<String>,
    includeDirectories: Vec<PathBuf>,
    numThreads: Option<usize>,
}

impl Config {
    fn default() -> Self {
        Self {
            includeDirectories: vec![],
            cachePriming_enable: false,
            cachePriming_numThreads: 0,
            compiler_arguments: vec![],
            compiler_path: None,
            compiler_onSave: false,
            eventsGameName: None,
            numThreads: None,
        }
    }
}

#[derive(Default)]
struct SourcepawnExtension {
    sourcepawn_lsp: Option<SourcepawnLsp>,
}

// impl SourcepawnExtension {
//     const ADAPTER_NAME: &str = "Sourcepawn";
// }

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
