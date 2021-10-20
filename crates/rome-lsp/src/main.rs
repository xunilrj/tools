use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use rome_server::{client::RomeClient, server::RomeBuilder};

struct RomeLsp {
    client: Client,
    rome: RomeClient,
}

impl RomeLsp {
    fn new(client: Client) -> Self {
        let rome = RomeBuilder::new();
        let rome = rome.start();
        Self { client, rome }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for RomeLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let msg = format!("{:?}", params);
        self.client.log_message(MessageType::Info, msg).await;

        let file = params.text_document_position_params.text_document.uri;

        if file.scheme() == "file" {
            let file = file.to_file_path().unwrap();
            let file = file.as_path().to_string_lossy().to_string();
            let pos = params.text_document_position_params.position;
            let line = pos.line as usize;
            let col = pos.character as usize;

            let r = self.rome.analyse_what(file, line, col).await;

            let title = MarkedString::String(r.title);
            Ok(Some(Hover {
                contents: HoverContents::Scalar(title),
                range: None,
            }))
        } else {
            Ok(None)
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(RomeLsp::new);
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
