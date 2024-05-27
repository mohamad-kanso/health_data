use crate::args::{
   DoctorCommand,
   DoctorSubcommand,
   CreateDoctor,
   UpdateDoctor,
   DeleteEntity
};

use crate::models::{Doctor,NewDoctor};
use crate::db::establish_connection;
use diesel::prelude::*;

pub fn handle_doctor_command(doctor: DoctorCommand) {
   let command = doctor.command;
   match command {
      DoctorSubcommand::Create(doctor) => create_doctor(doctor),
      DoctorSubcommand::Update(doctor) => update_doctor(doctor),
      DoctorSubcommand::Delete(delete_entity) => delete_doctor(delete_entity),
      DoctorSubcommand::Show => show_doctors()
   };
}

pub fn create_doctor(doctor: CreateDoctor){
   println!("Adding new doctor");
   use crate::schema::doctors;
   let conn = &mut establish_connection();

   let new_doctor = NewDoctor{
      id: doctor.id,
      name: &doctor.name,
      address: &doctor.address,
   };

   diesel::insert_into(doctors::table)
      .values(&new_doctor)
      .returning(Doctor::as_returning())
      .get_result(conn)
      .expect("Error adding new doctor");
}

pub fn update_doctor(doctor: UpdateDoctor){
   println!("Updating doctor");
   use crate::schema::doctors::dsl::*;
   let conn = &mut establish_connection();
   
   let db_doctor = Doctor {
      id: doctor.id,
      name: doctor.name,
      address: doctor.address,
   };

   diesel::update(doctors.find(doctor.id))
      .set(&db_doctor)
      .execute(conn)
      .expect("Error updating doctor");
}

pub fn delete_doctor(doctor: DeleteEntity){
   println!("Deleting doctor");
   use crate::schema::doctors::dsl::*;

   let conn = &mut establish_connection();

   diesel::delete(doctors.find(doctor.id))
      .execute(conn)
      .expect("Failed to delete");
}

pub fn show_doctors() {
   println!("Showing All doctors");
   use crate::schema::doctors::dsl::*;

   let conn = &mut establish_connection();

   let results = doctors
      .order(id)
      .limit(5)
      .select(Doctor::as_select())
      .load(conn)
      .expect("Error loading doctors");

   println!("Displaying {} doctors\n",results.len());

   for doctor in results {
      println!("******");
      println!("{}",doctor.name);
      println!("{}",doctor.address);
      println!("******");
      println!()
   }
}