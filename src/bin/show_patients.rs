use health_data::*;
use diesel::prelude::*;
use self::models::Patient;

fn main() {
   use self::schema::patients::dsl::*;

   let connection = &mut establish_connection();

   let results = patients
      .order(id)
      .limit(5)
      .select(Patient::as_select())
      .load(connection)
      .expect("Error loading patients");

   println!("Displating {} patients",results.len());

   for patient in results {
      println!("******");
      println!("{}",patient.name);
      println!("{}",patient.address);
      println!("doctor id: {}",patient.doctor_id);
      println!("******");
      println!()
   }
}  