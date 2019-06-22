#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Tera;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[get("/")]
fn home() -> Template {
    let context: HashMap<i8, i8> = HashMap::new();
    Template::render("index", context)
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
struct UserInput {
    projectname: String,
    pitchdate: String,
    intro1: String,
    intro2: String,
    intro3: String,
    people1: String,
    people2: String,
    col1_p1: String,
    col1_p2: String,
    col1_p3: String,
    col1_p4: String,
}

#[post("/", data = "<mainform>")]
fn handle_form(mainform: Form<UserInput>) -> Template {
    let context: HashMap<i8, i8> = HashMap::new();
    let t = Tera::new("templates/**/*").unwrap();
    write_out(
        &t.render("project.html.tera", &mainform.into_inner())
            .unwrap(),
        "static/output.html",
    );
    Template::render("index", context)
}

fn write_out(text: &str, path: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(text.as_bytes())?;
    Ok(())
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
