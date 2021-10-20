use flume::{bounded, Sender};

use crate::{analyser::*, server::*};

pub struct RomeClient(Sender<CommandEnvelope<RomeCommands, RomeResponse>>);

impl RomeClient {
    pub fn new(sender: Sender<CommandEnvelope<RomeCommands, RomeResponse>>) -> Self {
        Self(sender)
    }

    pub async fn watch_dir<S: AsRef<str>>(&self, dir: S) {
        let (callback, r) = bounded(1);
        let _ = self
            .0
            .send_async(CommandEnvelope {
                command: RomeCommands::WatchDirectory {
                    dir: dir.as_ref().to_string(),
                },
                callback,
            })
            .await;
        match r.recv_async().await {
            Ok(RomeResponse::None) => {}
            _ => todo!(),
        }
    }

    pub async fn project_start<S: AsRef<str>>(&self, language: S, root: S) {
        let (callback, r) = bounded(1);
        let _ = self
            .0
            .send_async(CommandEnvelope {
                command: RomeCommands::ProjectStart {
                    language: language.as_ref().to_string(),
                    root: root.as_ref().to_string(),
                },
                callback,
            })
            .await;
        match r.recv_async().await {
            Ok(RomeResponse::None) => {}
            _ => todo!(),
        }
    }

    pub async fn analyse_what<S: AsRef<str>>(
        &self,
        file: S,
        line: usize,
        col: usize,
    ) -> WhatResponse {
        let (callback, r) = bounded(1);
        let _ = self
            .0
            .send_async(CommandEnvelope {
                command: RomeCommands::AnalyserWhat {
                    file: file.as_ref().to_string(),
                    line,
                    col,
                },
                callback,
            })
            .await;
        match r.recv_async().await {
            Ok(RomeResponse::What(r)) => r,
            _ => todo!(),
        }
    }
}
