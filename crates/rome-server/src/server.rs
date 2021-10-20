use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crate::analyser::WhatResponse;
use crate::client::RomeClient;
use crate::db::*;
use crate::utils::normalize_path;
use crate::utils::spawn;
use flume::*;
use log::debug;

use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use salsa::ParallelDatabase;
use salsa::Snapshot;

// Will take more than a day! :P
pub struct RomeBuilder {}

#[derive(Debug)]
pub enum RomeCommands {
    // Filesystem Commands
    WatchDirectory {
        dir: String,
    },
    InvalidateFile {
        file: String,
    },

    // Project Commands
    ProjectStart {
        language: String,
        root: String,
    },

    // Semantic Commands
    AnalyserWhat {
        file: String,
        line: usize,
        col: usize,
    },
}

pub enum RomeResponse {
    None,
    What(WhatResponse),
}

pub struct CommandEnvelope<TCommand, TResponse> {
    pub command: TCommand,
    pub callback: Sender<TResponse>,
}

impl RomeBuilder {
    pub fn from_config(file: String) -> Self {
        todo!()
    }

    pub fn new() -> Self {
        Self {}
    }

    pub fn start(self) -> RomeClient {
        let (s, r) = unbounded();
        let sender = s.clone();
        let _ = spawn(async move {
            let mut db = RomeDb::default();

            loop {
                let envelope = r.recv_async().await;
                let sender = sender.clone();
                match envelope {
                    Ok(envelope) => Self::run(&mut db, envelope, sender),
                    Err(_) => todo!(),
                }
            }
        });

        RomeClient::new(s)
    }

    fn run(
        db: &mut RomeDb,
        envelope: CommandEnvelope<RomeCommands, RomeResponse>,
        sender: Sender<CommandEnvelope<RomeCommands, RomeResponse>>,
    ) {
        debug!(target: "server", "Received: {:?}", envelope.command);
        match envelope.command {
            RomeCommands::WatchDirectory { dir } => {
                let mut dir = normalize_path(dir);
                Self::start_watch_dir(dir, sender);
                envelope.callback.send(RomeResponse::None);
            }
            RomeCommands::InvalidateFile { file } => {
                let mut file = normalize_path(file);
                Self::invalidate_file(db, file);
                envelope.callback.send(RomeResponse::None);
            }
            RomeCommands::ProjectStart { language, root } => {
                let mut root = normalize_path(root);
                let entries = std::fs::read_dir(root).unwrap();
                for entry in entries.flatten() {
                    let path = entry.path();
                    debug!(target: "server", "Found: {:?}", path);
                    Self::invalidate_file(db, path);
                }
                envelope.callback.send(RomeResponse::None);
            }
            RomeCommands::AnalyserWhat { file, line, col } => {
                let mut file = normalize_path(file);
                let db = db.snapshot();
                let callback = envelope.callback;
                let _ = spawn(async move {
                    let r = crate::analyser::what(db, file, line, col).await.unwrap();
                    let _ = callback.send_async(RomeResponse::What(r)).await;
                });
            }
        };
    }

    fn invalidate_file<P: AsRef<Path>>(db: &mut RomeDb, file: P) {
        let meta = std::fs::metadata(&file).unwrap();
        let modified = meta.modified().unwrap();
        let modified = modified.elapsed().unwrap();

        let file = file.as_ref().to_string_lossy().to_string();
        db.set_file_version(file, modified.as_secs());
    }

    fn start_watch_dir(dir: String, sender: Sender<CommandEnvelope<RomeCommands, RomeResponse>>) {
        std::thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
            watcher.watch(dir, RecursiveMode::Recursive);
            loop {
                let command = match rx.recv() {
                    Ok(event) => {
                        let file = match event {
                            notify::DebouncedEvent::NoticeWrite(file) => Some(file),
                            notify::DebouncedEvent::Write(file) => Some(file),
                            notify::DebouncedEvent::NoticeRemove(_) => todo!(),
                            notify::DebouncedEvent::Remove(_) => todo!(),
                            notify::DebouncedEvent::Create(_) => todo!(),
                            notify::DebouncedEvent::Chmod(_) => todo!(),
                            notify::DebouncedEvent::Rename(_, _) => todo!(),
                            notify::DebouncedEvent::Rescan => todo!(),
                            notify::DebouncedEvent::Error(_, _) => todo!(),
                        };
                        if let Some(file) = file {
                            Some(RomeCommands::InvalidateFile {
                                file: file.to_string_lossy().to_string(),
                            })
                        } else {
                            None
                        }
                    }
                    Err(e) => todo!(),
                };

                if let Some(command) = command {
                    let (callback, r) = bounded(1);
                    sender.send(CommandEnvelope { command, callback });
                    r.recv();
                }
            }
        });
    }
}
