#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use elevator_pitch::{proj_io::write_out, UserInput};
use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Tera;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn home() -> Template {
    let context: HashMap<i8, i8> = HashMap::new();
    Template::render("index", context)
}

#[post("/", data = "<mainform>")]
fn handle_form(mainform: Form<UserInput>) -> Template {
    let context: HashMap<i8, i8> = HashMap::new();
    let t = Tera::new("templates/**/*").unwrap();
    write_out(
        &t.render("project.html.tera", &mainform.into_inner())
            .unwrap(),
        "static/outputs/output.html",
    )
    .expect("Failed to render template!");

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
