#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate pandoc;
extern crate rocket_contrib;

use elevator_pitch::{
    proj_io::{pandoc_out, write_out},
    UserInput,
};
use rocket::config::{Config, Environment};
use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Tera;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::env::var;
// pandoc

#[get("/")]
fn home() -> Template {
    let context: HashMap<i8, i8> = HashMap::new();
    Template::render("index", context)
}

pub fn process_form(input: UserInput) {
    // deal with form
    let new_template = Tera::new("templates/**/*");
    if let Ok(t) = new_template {
        let rendered = t.render("project.md.tera", &input).unwrap();
        write_out(&rendered, "static/outputs/output.md").expect("Failed to render template!");
        pandoc_out(&rendered, "static/outputs".to_owned(), input.outputformat)
    } else if let Err(e) = new_template {
        println!("Error locating template directory: {}", e);
    }
}

#[post("/", data = "<mainform>")]
fn handle_form(mainform: Form<UserInput>) -> Template {
    process_form(mainform.into_inner());

    // return main home page
    Template::render("index", UserInput::default())
}

fn make_config() -> Config {
    let port: u16 = var("PORT").unwrap().parse().unwrap();
    println!("{}", port);
    let config = Config::build(Environment::Production)
        .port(port)
        .finalize()
        .unwrap();
    config
}

fn main() {
    rocket::custom(make_config())
        .mount("/", routes![home, handle_form])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(Template::fairing())
        .launch();
}
