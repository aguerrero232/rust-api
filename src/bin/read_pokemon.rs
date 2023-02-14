use self::models::*;
use diesel::prelude::*;
use rust_api::*;

fn main() {
    use self::schema::pokemon::dsl::*;
    let connection = &mut establish_connection();
    let results = pokemon
        .limit(5)
        .load::<Pokemon>(connection)
        .expect("Error loading Pokemon...");

    println!("Displaying {} Pokemon", results.len());
    for item in results {
        println!("#{} {}", item.pokedex_number, item.name);
    }
}