use std::env;
use std::fs::File;
use markdown_gen::markdown::{AsMarkdown, Markdown};
use octocrab::Octocrab;
use pulldown_cmark::{html, Options, Parser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    //
    // let octocrab = Octocrab::builder().personal_token(github_token).build()?;

    // let file = File::create("C:/Users/Lucas/Development/vscode-schemas/README.md").unwrap();
    // let mut md = Markdown::new(file);
    //
    // println!("{:?}", Markdown::from("".as_markdown()));

    let markdown_input: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";
    println!("Parsing the following markdown string:\n{}", markdown_input);

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    let expected_html: &str =
        "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    assert_eq!(expected_html, &html_output);

    // Write result to stdout.
    println!("\nHTML output:\n{}", &html_output);
    Ok(())
}

