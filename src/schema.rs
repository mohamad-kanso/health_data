// @generated automatically by Diesel CLI.

diesel::table! {
    doctors (id) {
        id -> Int4,
        name -> Varchar,
        address -> Varchar,
    }
}

diesel::table! {
    patients (id) {
        id -> Int4,
        name -> Varchar,
        address -> Varchar,
        doctor_id -> Int4,
    }
}

diesel::joinable!(patients -> doctors (doctor_id));

diesel::allow_tables_to_appear_in_same_query!(
    doctors,
    patients,
);
