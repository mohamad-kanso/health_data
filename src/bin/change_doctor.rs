use health_data::*;
use diesel::prelude::*;
use std::env::args;
use std::io::stdin;
use self::models::Patient;

fn main() {
   use self::schema::patients::dsl::{patients,doctor_id};

   let mut doc_id = String::new();

   let id = args()
      .nth(1)
      .expect("changing doctor requires patient id")
      .parse::<i32>()
      .expect("Failed to parse id");

   println!("Input id of new doctor");
   stdin().read_line(&mut doc_id).expect("failed to read");
   let doc_id = doc_id.trim_end().parse::<i32>().expect("failed to parse doc id");

   let connection = &mut establish_connection();

   let patient = diesel::update(patients.find(id))
      .set(doctor_id.eq(doc_id))
      .returning(Patient::as_returning())
      .get_result(connection)
      .unwrap();
   println!("changed doc id to {}",patient.doctor_id)
}