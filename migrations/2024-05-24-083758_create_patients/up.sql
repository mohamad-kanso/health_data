CREATE TABLE patients(
   id SERIAL PRIMARY KEY,
   name VARCHAR NOT NULL,
   address VARCHAR NOT NULL,
   doctor_id INTEGER NOT NULL REFERENCES doctors(id)
)-- Your SQL goes here
