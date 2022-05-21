use flate2::read::GzDecoder;
use log::{debug, info};
use schema_lib::{
    clean_up_src_folder, octoduck::Octoduck, parse_folder_name, scan_for_ts_files,
    write_schema_list, SchemaList,
};
use std::any::Any;
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;
use std::{env, fs, io};
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
use swc_ecma_ast::{BindingIdent, Decl, Expr, Lit, ModuleDecl, ModuleItem, Pat, VarDeclKind};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax, TsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_module("schema_lib", log::LevelFilter::Trace)
        .filter_module("vscode_schemas", log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let extraction_dir: &Path = Path::new("../extraction");

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let octoduck = Octoduck::builder().personal_token(github_token).build()?;

    let repo = octoduck.repos("microsoft", "vscode");

    let last_release = repo.releases().get_last_release().await?;
    let last_release_tag_name = last_release.tag_name;
    info!("latest release name: {}", last_release_tag_name);

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
            Syntax::Typescript(Default::default()),
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

            if let ModuleItem::ModuleDecl(mdecl) = module_item {
                if let ModuleDecl::ExportDecl(edecl) = mdecl {
                    if let Decl::Var(vdecl) = edecl.decl {
                        if vdecl.kind == VarDeclKind::Const {
                            vdecl.decls.iter().for_each(|decl| {
                                let mut name: String = "".to_string();

                                if let Ident(bident) = &decl.name {
                                    name = bident.id.sym.to_string();
                                }

                                if name.contains("SchemaId") {
                                    if let Some(boxed_expr) = &decl.init {
                                        if let Expr::Lit(lit) = boxed_expr.unwrap_parens() {
                                            if let Lit::Str(lit_str) = lit {
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
            }
        }
        //     match module_item {
        //         ModuleItem::ModuleDecl(mdecl) => {
        //             match mdecl {
        //                 ModuleDecl::Import(_) => {}
        //                 ModuleDecl::ExportDecl(edecl) => {
        //                     match edecl.decl {
        //                         Decl::Class(_) => {}
        //                         Decl::Fn(_) => {}
        //                         Decl::Var(vdecl) => {
        //                             if vdecl.kind == VarDeclKind::Const {
        //
        //                                 // vdecl.decls.iter().for_each(|decl| {
        //                                 //     if let Some(ident) = &decl. {
        //                                 //         info!("{}", ident.sym);
        //                                 //     }
        //                                 // });
        //                             }
        //                         }
        //                         Decl::TsInterface(_) => {}
        //                         Decl::TsTypeAlias(_) => {}
        //                         Decl::TsEnum(_) => {}
        //                         Decl::TsModule(_) => {}
        //                     }
        //                 }
        //                 ModuleDecl::ExportNamed(_) => {}
        //                 ModuleDecl::ExportDefaultDecl(_) => {}
        //                 ModuleDecl::ExportDefaultExpr(_) => {}
        //                 ModuleDecl::ExportAll(_) => {}
        //                 ModuleDecl::TsImportEquals(_) => {}
        //                 ModuleDecl::TsExportAssignment(_) => {}
        //                 ModuleDecl::TsNamespaceExport(_) => {}
        //             }
        //         }
        //         ModuleItem::Stmt(_) => {}
        //     }
        //     // match module_item {
        //     //     ModuleItem::ModuleDecl(t) => {
        //     //         // Get type
        //     //
        //     //         debug!("{:?}", t);
        //     //     }
        //     //     ModuleItem::Stmt(_) => {}
        //     // }
        // }
        // let contents = serde_json::to_string_pretty(&_module.body).unwrap();
        // let mut file = File::create("../schema-list2.json").unwrap();
        // file.write_all(contents.as_bytes()).unwrap();

        // for e in parser.input().take() {
        //     info!("{:#?}", e);
        // }
    }

    // TODO uncomment this
    // clean_up_src_folder(src_folder.to_str().unwrap());

    // // println!("{:?}", repo.get().await?);
    // let mut release_page = repo.releases().list().per_page(10).send().await?;
    // let mut releases = release_page.take_items();
    //
    // while let Ok(Some(mut new_release)) = octoduck.get_page(&release_page.next).await {
    //     releases.extend(new_release.take_items());
    //
    //     release_page = new_release;
    // }
    //
    // for release in releases {
    //     println!("{:?}", release.tag_name);
    // }
    //
    // let last_two_releases = repo.releases().get_last_two_releases().await?;
    // // println!("{:?}", last_two_releases);
    // println!("{:?}", last_two_releases.names());

    // let mut compare_page = repo.compare().per_page(250).base("1.65.0").head("1.66.2").send().await?;
    //
    // let mut files = compare_page.take_items();
    // while let Ok(Some(mut new_compare)) = octoduck.get_page(&compare_page.next).await {
    //     files.extend(new_compare.take_items());
    //     compare_page = new_compare;
    // }

    // let mut compare_page = repo
    //     .compare("1.65.0".to_string(), "1.66.2".to_string())
    //     .list_commits()
    //     .per_page(250)
    //     .send()
    //     .await?;
    //
    // let mut files = compare_page.take_items();
    //
    // while let Ok(Some(mut new_compare)) = octoduck.get_page(&compare_page.next).await {
    //     files.extend(new_compare.take_items());
    //
    //     debug!("{:?}", new_compare.total_count);
    //
    //     compare_page = new_compare;
    // }

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

    // let repo = octocrab.repos("microsoft", "vscode");
    //
    // let last_two_releases = repo.releases().get_last_two_releases().await?;
    // // println!("{:?}", last_two_releases);
    // // println!("{:?}", last_two_releases.names());
    //
    // let last_two_releases_names = last_two_releases.names();
    //
    // let compared = repo.compare(last_two_releases_names.0, last_two_releases_names.1).await.unwrap();
    // println!("{:?}", compared);

    // let mut compare_page = repo.compare2().per_page(255).send(&octocrab, last_two_releases_names.0, last_two_releases_names.1).await?;
    // let mut compare_page = repo.compare().page(1u8).per_page(255).base("1.65.0").head("1.66.2").send().await?;

    // let toml_str = r#"
    // schemas = [
    //     "vscode://schemas/settings/default",
    //     "vscode://schemas/settings/folder",
    //     "vscode://schemas/settings/machine",
    //     "vscode://schemas/settings/resourceLanguage",
    //     "vscode://schemas/settings/user",
    //     "vscode://schemas/settings/workspace",
    //     "vscode://schemas/argv",
    //     "vscode://schemas/color-theme",
    //     "vscode://schemas/extensions",
    //     "vscode://schemas/global-snippets",
    //     "vscode://schemas/icon-theme",
    //     "vscode://schemas/icons",
    //     "vscode://schemas/ignoredSettings",
    //     "vscode://schemas/keybindings",
    //     "vscode://schemas/language-configuration",
    //     "vscode://schemas/launch",
    //     "vscode://schemas/product-icon-theme",
    //     "vscode://schemas/snippets",
    //     "vscode://schemas/tasks",
    //     "vscode://schemas/textmate-colors",
    //     "vscode://schemas/token-styling",
    //     "vscode://schemas/vscode-extensions",
    //     "vscode://schemas/workbench-colors",
    //     "vscode://schemas/workspaceConfig"
    // ]
    //
    // [versions_compared]
    // base = ""
    // head = """#;

    // let schema_list: SchemaList = read_schema_list();

    // let mut decoded: SchemaList = schema_lib::compare::read_schema_list();
    // println!("{:#?}", decoded);
    //
    // decoded.versions_compared.base = "69".to_string();
    // decoded.versions_compared.head = "420".to_string();
    // schema_lib::compare::write_schema_list(decoded);

    // schema_lib::docker::lmao().await;
    Ok(())
}
