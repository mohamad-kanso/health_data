use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use models::*;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL should be set");

    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}",database_url))
}

pub fn create_patient (conn: &mut PgConnection, name: &str, address: &str) -> Patient {
    use crate::schema::patients;

    let new_patient = NewPatient{name,address,doctor_id:0};

    diesel::insert_into(patients::table)
        .values(&new_patient)
        .returning(Patient::as_returning())
        .get_result(conn)
        .expect("Error adding new patient")
}

pub fn create_doctor (conn: &mut PgConnection, name: &str, address: &str) -> Doctor {
    use crate::schema::doctors;

    let new_doctor = NewDoctor{name,address};

    diesel::insert_into(doctors::table)
        .values(&new_doctor)
        .returning(Doctor::as_returning())
        .get_result(conn)
        .expect("Error adding doctor")
}