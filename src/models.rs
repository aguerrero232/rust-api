use diesel::{prelude::*};
use crate::schema::pokemon;

// #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query.
#[derive(Queryable)]
pub struct Pokemon {
    pub pokedex_number: i32,
    pub name: String,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub sp_attack: i32,
    pub sp_defense: i32,
    pub speed: i32,
    pub generation: i32,
    // pub moves: Vec<String>,
}

#[derive(Insertable)]
#[diesel(table_name = pokemon)]
pub struct NewPokemon<'a> {
    pub pokedex_number: &'a i32,
    pub name: &'a str,
    pub hp: &'a i32,
    pub attack: &'a i32,
    pub defense: &'a i32,
    pub sp_attack: &'a i32,
    pub sp_defense: &'a i32,
    pub speed: &'a i32,
    pub generation: &'a i32,
    // pub moves: Vec<String>,
}


