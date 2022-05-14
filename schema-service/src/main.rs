use std::env;
use env_logger::Target;
use log::{debug, info};
// use markdown_gen::markdown::{AsMarkdown, Markdown};
// use octocrab::Octocrab;
// use pulldown_cmark::{html, Options, Parser};
use schema_lib::{SchemaList};
// use schema_lib::releases::ReleaseHandlerExt;
// use schema_lib::repo::RepoHandlerExt;

use schema_lib::{
    octoduck::Octoduck
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_module("schema_lib", log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let octoduck = Octoduck::builder().personal_token(github_token).build()?;

    let repo = octoduck.repos("microsoft", "vscode");


    // println!("{:?}", repo.get().await?);

    let last_two_releases = repo.releases().get_last_two_releases().await?;
    // println!("{:?}", last_two_releases);
    println!("{:?}", last_two_releases.names());

    let mut compare_page = repo.compare().page(1u8).per_page(30).base("1.65.0").head("1.66.2").send().await?;

    let mut files = compare_page.take_items();
    while let Ok(Some(mut new_compare)) = octoduck.get_page(&compare_page.next).await {
        files.append(&mut new_compare.take_items());
        compare_page = new_compare;
    }
    debug!("{:?}", files.len());

    // let file = File::create("C:/Users/Yepper/Yepper/vscode-schemas/README.md").unwrap();
    // let mut md = Markdown::new(file);
    //
    // println!("{:?}", Markdown::from("".as_markdown()));

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

    let schema_list: SchemaList = schema_lib::compare::read_schema_list();


    // let mut decoded: SchemaList = schema_lib::compare::read_schema_list();
    // println!("{:#?}", decoded);
    //
    // decoded.versions_compared.base = "69".to_string();
    // decoded.versions_compared.head = "420".to_string();
    // schema_lib::compare::write_schema_list(decoded);

    // schema_lib::docker::lmao().await;
    Ok(())
}

