CREATE TABLE patients(
   id INTEGER PRIMARY KEY,
   name VARCHAR NOT NULL,
   address VARCHAR NOT NULL,
   doctor_id INTEGER REFERENCES doctors(id) DEFAULT '0'
)-- Your SQL goes here
