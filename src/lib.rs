#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

pub mod proj_io;
pub mod schema;

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct UserInput {
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
