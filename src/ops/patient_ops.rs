use crate::args::{
   PatientCommand,
   PatientSubcommand,
   CreatePatient,
   UpdatePatient,
   DeleteEntity
};

use crate::models::{Patient,NewPatient};
use crate::db::establish_connection;
use diesel::prelude::*;

pub fn handle_patient_command(patient: PatientCommand) {
   let command = patient.command;
   match command {
      PatientSubcommand::Create(patient) => create_patient(patient),
      PatientSubcommand::Update(patient) => update_patient(patient),
      PatientSubcommand::Delete(delete_entity) => delete_patient(delete_entity),
      PatientSubcommand::Show => show_patients()
   };
}

pub fn create_patient(patient: CreatePatient){
   println!("Adding new patient");
   use crate::schema::patients;
   let conn = &mut establish_connection();

   let new_patient = NewPatient{
      id: patient.id,
      name: &patient.name,
      address: &patient.address,
      doctor_id: patient.doctor_id,
   };

   diesel::insert_into(patients::table)
      .values(&new_patient)
      .returning(Patient::as_returning())
      .get_result(conn)
      .expect("Error adding new patient");
}

pub fn update_patient(patient: UpdatePatient){
   println!("Updating patient");
   use crate::schema::patients::dsl::*;
   let conn = &mut establish_connection();
   
   let db_patient = Patient {
      id: patient.id,
      name: patient.name,
      address: patient.address,
      doctor_id: patient.doctor_id
   };

   diesel::update(patients.find(patient.id))
      .set(&db_patient)
      .execute(conn)
      .expect("Error updating patient");
}

pub fn delete_patient(patient: DeleteEntity){
   println!("Deleting patient");
   use crate::schema::patients::dsl::*;

   let conn = &mut establish_connection();

   diesel::delete(patients.find(patient.id))
      .execute(conn)
      .expect("Failed to delete");
}

pub fn show_patients() {
   println!("Showing All Patients");
   use crate::schema::patients::dsl::*;

   let conn = &mut establish_connection();

   let results = patients
      .order(id)
      .limit(5)
      .select(Patient::as_select())
      .load(conn)
      .expect("Error loading patients");

   println!("Displaying {} patients\n",results.len());

   for patient in results {
      println!("******");
      println!("{}",patient.name);
      println!("{}",patient.address);
      println!("doctor id: {}",patient.doctor_id);
      println!("******");
      println!()
   }
}