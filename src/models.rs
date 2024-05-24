use diesel::prelude::*;
use crate::schema::{doctors,patients};

#[derive(Queryable,Selectable,Debug,PartialEq)]
#[diesel(table_name = doctors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Doctor {
    pub id: i32,
    pub name: String,
    pub address: String,
}

#[derive(Insertable)]
#[diesel(table_name = doctors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewDoctor<'a> {
    pub name: &'a str,
    pub address: &'a str
}

#[derive(Queryable,Selectable,Associations,Debug,PartialEq)]
#[diesel(belongs_to(Doctor))]
#[diesel(table_name = patients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub doctor_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = patients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPatient<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub doctor_id: i32,
}