use flate2::read::GzDecoder;
use log::info;
use schema_lib::{
    clean_up_src_folder, octoduck::Octoduck, parse_folder_name, read_schema_list,
    scan_for_ts_files, write_schema_list, SchemaList,
};
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::{env, io};
use tar::Archive;

// use markdown_gen::markdown::{AsMarkdown, Markdown};
// use octocrab::Octocrab;
// use pulldown_cmark::{html, Options, Parser};
// use schema_lib::releases::ReleaseHandlerExt;
// use schema_lib::repo::RepoHandlerExt;

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
use swc_ecma_ast::{BindingIdent, BlockStmt, Decl, Expr, Lit, ModuleDecl, ModuleItem, Pat, Stmt, VarDeclKind};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax, TsConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_module("schema_lib", log::LevelFilter::Trace)
        .filter_module("vscode_schemas", log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let mut schema_list: SchemaList = read_schema_list();

    let extraction_dir: &Path = Path::new("../extraction");

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let octoduck = Octoduck::builder().personal_token(github_token).build()?;

    let repo = octoduck.repos("microsoft", "vscode");

    let last_release = repo.releases().get_last_release().await?;
    let last_release_tag_name = last_release.tag_name;
    info!("latest release name: {}", last_release_tag_name);

    if schema_list.last_release == last_release_tag_name {
        info!("no new releases");
        return Ok(());
    }

    let mut sha = repo.get_latest_commit_sha().await?;
    sha = sha[0..7].to_string();
    info!("latest commit: {}", sha);

    let unpack_name = parse_folder_name(&sha);
    info!("unpack name: {}", unpack_name);

    let src_folder = extraction_dir.join(unpack_name);

    let default_branch = repo.default_branch().await?;
    info!("default branch: {}", default_branch);

    if !Path::new(src_folder.to_str().unwrap()).exists() {
        let res = repo.download_tarball(default_branch).await?;
        let mut file = File::create(extraction_dir.join("vscode.tar.gz"))?;

        let bytes = res.bytes().await.expect("failed to read bytes");

        let mut content = Cursor::new(bytes);
        io::copy(&mut content, &mut file).expect("failed to write file");

        let tar_gz = File::open(extraction_dir.join("vscode.tar.gz"))?;

        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(extraction_dir.join("."))?;
    }

    let ts_files = scan_for_ts_files(src_folder.join("src").to_str().unwrap())?;
    info!("ts files: {:?}", ts_files.len());

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let mut schema_paths = Vec::<String>::new();

    for item in ts_files {
        // let contents = fs::read_to_string(item).expect("failed to read file");
        //
        // let lines = contents.lines();
        //
        // lines.for_each(| line| {
        //    debug!("{}", line);
        // });

        // let fm = cm.new_source_file(
        //     FileName::Custom("test.ts".into()),
        //     "export const configurationDefaultsSchemaId = 'vscode://schemas/settings/configurationDefaults';".into(),
        // );
        //
        let fm = cm
            .load_file(Path::new(item.as_str()))
            .expect("failed to load file");
        info!("Loaded {}", item);

        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                decorators: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let capturing = Capturing::new(lexer);

        let mut parser = Parser::new_from(capturing);

        for e in parser.take_errors() {
            e.into_diagnostic(&handler).emit();
        }

        let module = parser
            .parse_typescript_module()
            .map_err(|e| e.into_diagnostic(&handler).emit())
            .expect("Failed to parse module.");

        for module_item in module.body {
            // if let Ident(name) = &decl.name {
            // }

            match module_item {
                ModuleItem::ModuleDecl(module_decl) => {
                    if let ModuleDecl::ExportDecl(export_decl) = module_decl {
                        if let Decl::Var(var_decl) = export_decl.decl {
                            if var_decl.kind == VarDeclKind::Const {
                                var_decl.decls.iter().for_each(|decl| {
                                    let mut name: String = "".to_string();

                                    if let Ident(bident) = &decl.name {
                                        name = bident.id.sym.to_string();
                                    }

                                    if name.to_lowercase().contains("schemaid") {
                                        if let Some(boxed_expr) = &decl.init {
                                            if let Expr::Lit(lit) = boxed_expr.unwrap_parens() {
                                                if let Lit::Str(lit_str) = lit {
                                                    schema_paths.push(lit_str.value.to_string());
                                                    info!(
                                                        "value !!!!! {:?}",
                                                        lit_str.value.to_string()
                                                    );
                                                }
                                            }
                                        }
                                    }
                                });
                            }
                        } else if let Decl::Fn(fn_decl) = export_decl.decl {
                            if let Some(BlockStmt { stmts, ..}) = fn_decl.function.body {
                                stmts.iter().for_each(|stmt| {
                                    if let Stmt::Decl(decl) = stmt {
                                        if let Decl::Var(var_decl) = decl {
                                            if var_decl.kind == VarDeclKind::Const {
                                                var_decl.decls.iter().for_each(|decl| {
                                                    let mut name: String = "".to_string();

                                                    if let Ident(bident) = &decl.name {
                                                        name = bident.id.sym.to_string();
                                                    }

                                                    if name.to_lowercase().contains("schemaid") {
                                                        if let Some(boxed_expr) = &decl.init {
                                                            if let Expr::Lit(lit) = boxed_expr.unwrap_parens() {
                                                                if let Lit::Str(lit_str) = lit {
                                                                    schema_paths.push(lit_str.value.to_string());
                                                                    info!(
                                                                        "value !!!!! {:?}",
                                                                        lit_str.value.to_string()
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
                ModuleItem::Stmt(stmt) => {
                    if let Stmt::Decl(decl) = stmt {
                        if let Decl::Var(var_decl) = decl {
                            if var_decl.kind == VarDeclKind::Const {
                                var_decl.decls.iter().for_each(|decl| {
                                    let mut name: String = "".to_string();

                                    if let Ident(bident) = &decl.name {
                                        name = bident.id.sym.to_string();
                                    }

                                    if name.to_lowercase().contains("schemaid") {
                                        if let Some(boxed_expr) = &decl.init {
                                            if let Expr::Lit(lit) = boxed_expr.unwrap_parens() {
                                                if let Lit::Str(lit_str) = lit {
                                                    schema_paths.push(lit_str.value.to_string());
                                                    info!(
                                                        "value !!!!! {:?}",
                                                        lit_str.value.to_string()
                                                    );
                                                }
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    } else if let Stmt::Expr(expr) = stmt {
                         if let Expr::Call(call_expr) = &expr.expr.unwrap_parens() {
                             call_expr.args.iter().for_each(|arg| {
                                 if let Expr::Lit(lit) = arg.expr.unwrap_parens() {
                                     if let Lit::Str(lit_str) = lit {
                                         schema_paths.push(lit_str.value.to_string());
                                         info!(
                                             "value !!!!! {:?}",
                                             lit_str.value.to_string()
                                         );
                                     }
                                 }
                             });
                         }
                    }
                }
            }
        }
    }

    // let markdown_input: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";
    // println!("Parsing the following markdown string:\n{}", markdown_input);
    //
    // // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // // and we therefore must enable it explicitly.
    // let mut options = Options::empty();
    // options.insert(Options::ENABLE_STRIKETHROUGH);
    // let parser = Parser::new_ext(markdown_input, options);
    //
    // // Write to String buffer.
    // let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    // html::push_html(&mut html_output, parser);
    //
    // // Check that the output is what we expected.
    // let expected_html: &str =
    //     "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    // assert_eq!(expected_html, &html_output);
    //
    // // Write result to stdout.
    // println!("\nHTML output:\n{}", &html_output);

    // schema_lib::docker::lmao().await;
    info!("SCHEMAS = {:?}", schema_paths);

    schema_list = SchemaList {
        last_release: last_release_tag_name,
        schemas: schema_paths,
    };

    // write_schema_list(schema_list);
    // clean_up_src_folder(src_folder.to_str().unwrap());
    Ok(())
}
