extern crate diesel;
extern crate dotenvy;

mod args;
mod ops;
mod db;
mod schema;
mod models;

use ops::doctor_ops::handle_doctor_command;
use ops::patient_ops::handle_patient_command;
use args::HealthArgs;
use args::EntityType;
use clap::Parser;

fn main () {
   let args = HealthArgs::parse();

   match args.entity_type {
      EntityType::Patient(patient) => handle_patient_command(patient),
      EntityType::Doctor(doctor) => handle_doctor_command(doctor)
   };
}