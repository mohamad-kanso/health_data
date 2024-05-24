use self::models::Doctor;
use diesel::prelude::*;
use health_data::*;

fn main() {
   use self::schema::doctors::dsl::*;

   let connection = &mut establish_connection();
   let results = doctors
      .order(id)
      .limit(5)
      .select(Doctor::as_select())
      .load(connection)
      .expect("Error loading Doctors");

   println!("Displaying {} doctors",results.len());
   for doctor in results {
      println!("{}",doctor.name);
      println!("{}",doctor.address);
   }
}