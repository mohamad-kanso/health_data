use health_data::{create_patient, establish_connection};
use std::io::stdin;

fn main() {
   let connection = &mut establish_connection();

   let mut id = String::new();
   let mut name = String::new();
   let mut address = String::new();
   let mut doctor_id = String::new();

   println!("what is the id of the patient?");
   stdin().read_line(&mut id).unwrap();
   let id = id.trim_end().parse::<i32>().unwrap();

   println!("what is the name of the patient?");
   stdin().read_line(&mut name).unwrap();
   let name = name.trim_end();

   println!("what is their address?");
   stdin().read_line(&mut address).unwrap();
   let address = address.trim_end();

   println!("what is their doctor's id");
   stdin().read_line(&mut doctor_id).unwrap();
   let doctor_id = doctor_id.trim_end().parse::<i32>().unwrap();

   let patient = create_patient(connection,id, name, address,doctor_id);
   println!("Saved patient {} with id {}",patient.name,patient.id);
}