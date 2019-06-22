#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate pandoc;
extern crate rocket_contrib;

use elevator_pitch::{proj_io::write_out, UserInput};
use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Tera;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

// pandoc

#[get("/")]
fn home() -> Template {
    let context: HashMap<i8, i8> = HashMap::new();
    Template::render("index", context)
}

pub fn process_form(input: UserInput) {
    // deal with form
    let t = Tera::new("templates/**/*").unwrap();
    let rendered = t.render("project.md.tera", &input).unwrap();

    write_out(&rendered, "static/outputs/output.md").expect("Failed to render template!");
    convert(&rendered, "static/outputs/test.html".to_owned())
}

pub fn convert(input: &String, output_path: String) {
    let mut p = pandoc::new();
    p.set_input_format(pandoc::InputFormat::MarkdownGithub, vec![]);
    p.set_input(pandoc::InputKind::Pipe(input.to_string()));

    p.set_output_format(pandoc::OutputFormat::Revealjs, vec![]);
    p.add_options(&vec![
        pandoc::PandocOption::Standalone,
        pandoc::PandocOption::SlideLevel(2),
    ]);
    p.set_output(pandoc::OutputKind::File(output_path));

    match p.execute() {
        Ok(_) => println!("Form processed and converted!"),
        Err(_) => println!("Could not process form!"),
    }
}

#[post("/", data = "<mainform>")]
fn handle_form(mainform: Form<UserInput>) -> Template {
    process_form(mainform.into_inner());

    // return main home page
    let context: HashMap<i8, i8> = HashMap::new();
    Template::render("index", context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![home, handle_form])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(Template::fairing())
        .launch();
}
