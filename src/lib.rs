use crate::zed::LanguageServerId;
use zed_extension_api as zed;
use zed_extension_api::Result;

struct Hl7V2;

impl zed::Extension for Hl7V2 {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: String::from(
                "/Users/jessekruse/other_Code/rustProjects/hl7_v2_lsp/target/debug/hl7_v2_lsp",
            ),
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

zed::register_extension!(Hl7V2);
