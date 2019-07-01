extern crate pandoc;
use std::fs::File;
use std::io::prelude::*;

pub fn write_out(text: &str, path: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(text.as_bytes())?;
    Ok(())
}

trait FromSelection {
    fn parse(input: &str) -> (String, pandoc::OutputFormat);
}

impl FromSelection for pandoc::OutputFormat {
    fn parse(input: &str) -> (String, pandoc::OutputFormat) {
        use pandoc::OutputFormat::{Docx, Latex, Revealjs};
        match input {
            "reveal" => ("output.html".to_string(), Revealjs),
            "docx" => ("output.docx".to_string(), Docx),
            "pdf" => ("output.pdf".to_string(), Latex),
            _ => ("output.not_found".to_string(), Revealjs),
        }
    }
}

pub fn pandoc_out(input: &String, output_path: String, format_string: String) {
    // TODO: handle paths in a better way
    use pandoc::PandocOption::{SlideLevel, Standalone, Var};
    let output_format = pandoc::OutputFormat::parse(&format_string);
    let full_path = format!("{}/{}", output_path, output_format.0);

    let mut p = pandoc::new();
    p.set_input_format(pandoc::InputFormat::MarkdownGithub, vec![]);
    p.set_input(pandoc::InputKind::Pipe(input.to_string()));
    p.set_output(pandoc::OutputKind::File(full_path));
    p.set_output_format(output_format.1, vec![]);
    p.add_options(&vec![
        Standalone,
        SlideLevel(2),
        Var(String::from("theme"), Some(String::from("simple"))),
        Var(
            String::from("revealjs-url"),
            Some(String::from("https://revealjs.com")),
        ),
    ]);
    match p.execute() {
        Ok(_) => println!("Form processed and converted!"),
        Err(e) => println!("Could not process form! Err: {}", e),
    }
}
