#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use std::fs::{metadata, File};
use std::io::{Read, Write};
use std::path::Path;
use url::Url;
use walkdir::WalkDir;

pub mod docker;
pub mod error;
pub mod octoduck;
use regex::Regex;

#[macro_use]
extern crate swc_common;
extern crate swc_ecma_parser;
use swc_common::sync::Lrc;
use swc_common::util::take::Take;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecma_ast::Pat::Ident;
use swc_ecma_ast::{
    BindingIdent, BlockStmt, Decl, Expr, FnDecl, Lit, ModuleDecl, ModuleItem, Pat, Stmt, VarDecl,
    VarDeclKind,
};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax, TsConfig};

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SchemaList {
    pub last_release: String,
    pub schemas: Vec<String>,
}

pub fn read_schema_list() -> SchemaList {
    let mut file = File::open("../schema-list.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let schema_list: SchemaList = serde_json::from_str(&contents).unwrap();
    schema_list
}

pub fn write_schema_list(schema_list: SchemaList) {
    let contents = serde_json::to_string_pretty(&schema_list).unwrap();
    let mut file = File::create("../schema-list.json").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn parse_folder_name(sha: &str) -> String {
    return "microsoft-vscode-".to_owned() + &sha;
}

pub fn clean_up_src_folder(folder_name: &str) {
    let path = std::path::Path::new(folder_name);
    if path.exists() {
        std::fs::remove_dir_all(folder_name).unwrap();
    }
}

pub fn scan_for_ts_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        let metadata = metadata(&path)?;
        if metadata.is_file() {
            if path.extension().unwrap() == "ts" {
                // if path.to_str().unwrap().to_owned() != "../extraction\\microsoft-vscode-93ec6bd\\src\\vs\\platform\\configuration\\common\\configurationRegistry.ts" {
                //      continue;
                // }

                // if path.to_str().unwrap().to_owned() != "../extraction\\microsoft-vscode-93ec6bd\\src\\vs\\workbench\\services\\extensions\\common\\extensionsRegistry.ts" {
                //     continue;
                // }
                //
                // if path.to_str().unwrap().to_owned() != "../extraction\\microsoft-vscode-93ec6bd\\src\\vs\\workbench\\electron-sandbox\\desktop.contribution.ts" {
                //     continue;
                // }

                // if path.to_str().unwrap().to_owned() != "../extraction\\microsoft-vscode-93ec6bd\\src\\vs\\platform\\userDataSync\\common\\userDataSync.ts" {
                //     continue;
                // }
                //
                // if path.to_str().unwrap().to_owned() != "../extraction\\microsoft-vscode-93ec6bd\\src\\vs\\workbench\\api\\common\\configurationExtensionPoint.ts" {
                //     continue;
                // }
                if path.to_str().unwrap().to_owned() != "../extraction\\microsoft-vscode-528ee1a\\src\\vs\\workbench\\contrib\\notebook\\browser\\notebook.contribution.ts" {
                    continue;
                }

                files.push(path.to_str().unwrap().to_owned())
                // debug!("{}", path.display());
            }
        }
    }

    Ok(files)
}

pub fn parse_variable_string(schema_paths: &mut Vec<String>, var_decl: &VarDecl) {
    if var_decl.kind == VarDeclKind::Const {
        var_decl.decls.iter().for_each(|decl| {
            let mut name: String = "".to_string();

            if let Ident(biding_ident) = &decl.name {
                name = biding_ident.id.sym.to_string();
            }

            if name.to_lowercase().contains("schemaid") {
                if let Some(boxed_expr) = &decl.init {
                    if let Expr::Lit(lit) = boxed_expr.unwrap_parens() {
                        if let Lit::Str(lit_str) = lit {
                            let val = lit_str.value.to_string();

                            if val.to_lowercase().contains("vscode://schema") {
                                schema_paths.push(lit_str.value.to_string());
                                info!("found value -> {:?}", val);
                            }
                        }
                    }
                }
            }
        });
    }
}
