use std::{path::PathBuf, sync::Arc};

use rslint_core::{Directive, DirectiveError, DirectiveParser};
use rslint_parser::ast::Module;
use salsa::Snapshot;

#[derive(Debug)]
pub struct Project {
    files: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct ParsedJavascript {
    pub directives: Vec<Directive>,
    pub errors: Vec<DirectiveError>,
}

#[derive(Debug)]
pub enum ParsedFile {
    Javascript(ParsedJavascript),
}

impl ParsedFile {
    pub fn as_javascript(&self) -> &ParsedJavascript {
        match self {
            ParsedFile::Javascript(j) => j,
        }
    }
}

impl Eq for ParsedFile {}

impl PartialEq for ParsedFile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Javascript(_), Self::Javascript(_)) => false, //TODO
        }
    }
}

fn parse_file(db: &dyn RomeQueries, file: String) -> Arc<ParsedFile> {
    let version = db.file_version(file.clone());
    let code = db.file_contents(file.clone());
    let code = String::from_utf8(code).unwrap();

    let mut files = rslint_errors::file::SimpleFiles::new();
    let file_id = files.add(file.clone(), code.clone());
    let mut file = rslint_core::File::from_string(code, rslint_parser::FileKind::Module, file);
    file.id = file_id;

    let (parsing_errors, root) = file.parse_with_errors();
    let res = DirectiveParser::new(root.clone(), &file).get_file_directives();

    let directives = dbg!(res.directives);
    let errors = dbg!(res.diagnostics);

    Arc::new(ParsedFile::Javascript(ParsedJavascript {
        directives,
        errors,
    }))
}

fn file_contents(db: &dyn RomeQueries, file: String) -> Vec<u8> {
    std::fs::read(file).unwrap()
}

#[salsa::query_group(RomeStorage)]
pub trait RomeQueries: salsa::Database {
    #[salsa::transparent]
    fn file_contents(&self, file: String) -> Vec<u8>;

    #[salsa::input]
    fn project(&self) -> Arc<Project>;
    #[salsa::input]
    fn file_version(&self, file: String) -> u64;

    fn parse_file(&self, file: String) -> Arc<ParsedFile>;
}

#[salsa::database(RomeStorage)]
#[derive(Default)]
pub struct RomeDb {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for RomeDb {}

impl salsa::ParallelDatabase for RomeDb {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(RomeDb {
            storage: self.storage.snapshot(),
        })
    }
}
