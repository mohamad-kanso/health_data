use clap::{Args,Parser,Subcommand};

#[derive(Debug, Parser)]
#[clap(author,version, about)]
pub struct HealthArgs {
   #[clap(subcommand)]
   pub entity_type: EntityType,
}

#[derive(Debug,Subcommand)]
pub enum EntityType{
   Patient(PatientCommand),
   Doctor(DoctorCommand),
}

#[derive(Debug,Args)]
pub struct PatientCommand {
   #[clap(subcommand)]
   pub command: PatientSubcommand,
}

#[derive(Debug,Subcommand)]
pub enum PatientSubcommand {
   Create(CreatePatient),
   Update(UpdatePatient),
   Delete(DeleteEntity),
   Show,
}

#[derive(Debug,Args)]
pub struct CreatePatient {
   pub id: i32,
   pub name: String,
   pub address: String,
   pub doctor_id: i32,
}

#[derive(Debug,Args)]
pub struct UpdatePatient {
   pub id: i32,
   pub name: String,
   pub address: String,
   pub doctor_id: i32,
}

#[derive(Debug,Args)]
pub struct DeleteEntity {
   pub id: i32,
}

#[derive(Debug,Args)]
pub struct DoctorCommand {
   #[clap(subcommand)]
   pub command: DoctorSubcommand,
}

#[derive(Debug,Subcommand)]
pub enum DoctorSubcommand {
   Create(CreateDoctor),
   Update(UpdateDoctor),
   Delete(DeleteEntity),
   Show,
}

#[derive(Debug,Args)]
pub struct CreateDoctor {
   pub id: i32,
   pub name: String,
   pub address: String,
}

#[derive(Debug,Args)]
pub struct UpdateDoctor {
   pub id: i32,
   pub name: String,
   pub address: String,
}