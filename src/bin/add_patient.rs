use health_data::{create_patient, establish_connection};
use std::io::stdin;

fn main() {
   let connection = &mut establish_connection();

   let mut name = String::new();
   let mut address = String::new();

   println!("what is the name of the patient?");
   stdin().read_line(&mut name).unwrap();
   let name = name.trim_end();

   println!("what is their address?");
   stdin().read_line(&mut address).unwrap();
   let address = address.trim_end();

   let patient = create_patient(connection, name, address);
   println!("Saved patient {} with id {}",patient.name,patient.id);
}