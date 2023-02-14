// @generated automatically by Diesel CLI.

diesel::table! {
    pokemon (pokedex_number) {
        pokedex_number -> Int4,
        name -> Varchar,
        hp -> Int4,
        attack -> Int4,
        defense -> Int4,
        sp_attack -> Int4,
        sp_defense -> Int4,
        speed -> Int4,
        generation -> Int4,
    }
}