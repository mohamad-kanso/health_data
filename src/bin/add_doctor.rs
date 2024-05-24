use health_data::*;
use std::io::stdin;

fn main() {
   let connection = &mut establish_connection();

   let mut name = String::new();
   let mut address = String::new();

   println!("what is the name of the doctor?");
   stdin().read_line(&mut name).unwrap();
   let name = name.trim_end();

   println!("what is their address?");
   stdin().read_line(&mut address).unwrap();
   let address = address.trim_end();

   let doctor = create_doctor(connection, name, address);
   println!("Saved patient {} with id {}",doctor.name,doctor.id);
}