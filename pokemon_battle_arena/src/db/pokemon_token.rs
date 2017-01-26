extern crate csv;

use super::pokemon_model;
use super::natures;
use super::enums;
use super::stats;

#[derive(Debug, Clone)]
pub struct PokemonToken {
    pokedex_id: usize,
    pub name: String,
    gender: enums::Gender,
    pub type_one: enums::types,
    pub type_two: enums::types,
    nature: natures::Nature,
    base_stats: stats::Stats,
    current_stats: stats::Stats,
    mega_evolution: Box<Option<pokemon_model::PokemonModel>>,
}


impl PokemonToken {
    pub fn from_model(model: pokemon_model::PokemonModel) -> PokemonToken {
        PokemonToken {
            pokedex_id: model.pokedex_id,
            name: model.name,
            gender: enums::Gender::Male,
            type_one: model.type_one,
            type_two: model.type_two,
            nature: natures::Nature::get_random_nature(),
            base_stats: model.base_stats.clone(),
            current_stats: model.base_stats,
            mega_evolution: model.mega_evolution,
        }
    }

    pub fn get_mega(&self) -> Option<PokemonToken> {
        if self.mega_evolution.is_some() {
            return Some(PokemonToken::from_model(self.mega_evolution.clone().unwrap()));
        }
        None
    }
}
