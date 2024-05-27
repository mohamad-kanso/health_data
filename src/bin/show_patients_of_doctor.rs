use health_data::{models::{Doctor, Patient}, *};
use diesel::prelude::*;
use std::io::stdin;

fn main() {
   use crate::schema::*;
   
   let connection = &mut establish_connection();

   let mut doctor_id = String::new();

   println!("whose doctor's patients do you want to see");
   stdin().read_line(&mut doctor_id).unwrap();
   let doctor_id = doctor_id.trim_end().parse::<i32>().expect("failed to parse id");

   let doc = doctors::table
      .filter(doctors::id.eq(doctor_id))
      .select(Doctor::as_select())
      .get_result(connection)
      .expect("failed to retrieve doctor");

   let patients = Patient::belonging_to(&doc)
      .select(Patient::as_select())
      .load(connection)
      .expect("failed to retrieve patients");

   println!("\nPatients for doctor: {}\n",doc.name);
   for patient in patients {
      println!("name: {}",patient.name);
      println!("address: {}",patient.address);
      println!();
   }
}