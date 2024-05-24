use health_data::*;
use diesel::prelude::*;
use std::env::args;

fn main (){
   use self::schema::patients::dsl::*;

   let target = args().nth(1).expect("expected a target to match against");
   let pattern = format!("%{}%",target);

   let connection = &mut establish_connection();
   let num_deleted = diesel::delete(patients.filter(name.like(pattern)))
      .execute(connection)
      .expect("Error deleting patients");

   println!("Deleted {} patients",num_deleted);
}