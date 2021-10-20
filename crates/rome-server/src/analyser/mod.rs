use std::{path::PathBuf, sync::Arc};

use crate::db::*;
use flume::*;
use rslint_parser::ast::Module;
use salsa::{ParallelDatabase, Snapshot};

#[derive(Debug)]
pub struct WhatResponse {
    pub title: String,
}

pub async fn what(
    db: Snapshot<RomeDb>,
    file: String,
    line: usize,
    col: usize,
) -> Result<WhatResponse, ()> {
    

    let parsed = db.parse_file(file);
    let parsed = parsed.as_javascript();

    Ok(WhatResponse {
        title: format!("{}", parsed.errors.len()),
    })
}
