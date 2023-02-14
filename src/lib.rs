use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::*;
use schema::pokemon;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_pokemon<'a>(conn: &mut PgConnection, pokedex_number: &'a i32, name: &'a str, hp: &'a i32, attack: &'a i32, defense: &'a i32, sp_attack: &'a i32, sp_defense: &'a i32, speed: &'a i32, generation: &'a i32) -> Pokemon {

    let new_pokemon = NewPokemon {
        pokedex_number,
        name,
        hp,
        attack,
        defense,
        sp_attack,
        sp_defense,
        speed,
        generation,
    };

    diesel::insert_into(pokemon::table)
        .values(&new_pokemon)
        .get_result(conn)
        .expect("Error saving new Pokemon")
}

pub fn read_pokemon(conn: &mut PgConnection) -> Vec<Pokemon> {
    use schema::pokemon::dsl::*;

    pokemon
        .limit(5)
        .load::<Pokemon>(conn)
        .expect("Error loading Pokemon")
}