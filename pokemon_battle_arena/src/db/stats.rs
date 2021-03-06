use super::determinant_values::Dv;
use super::enums;
use super::natures;
use super::pokemon_model::PokemonModel;


/// Contains the main stats for every Pokemon.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stats {
    hp: u16,
    attack: u16,
    defense: u16,
    special_attack: u16,
    special_defense: u16,
    speed: u16,
    accuracy: u16,
    evasion: u16,
}

impl Stats {
    /// Is used to calculate the stat when converting a Pokemon Model in a Pokemon Token.
    pub fn calculate_stats(model: PokemonModel,
                           dv: Dv,
                           nature: natures::Nature,
                           level: u16)
                           -> Stats {
        let hp = ((((2.0 * model.get_stats().get_stat(&enums::Stats::Hp) as f32 +
                     (dv.get_dv(enums::Stats::Hp) as f32 / 4.0)) as
                    f32 * level as f32) / 100.0) + level as f32 + 10.0) as u16;

        fn stat_formula(base: u16,
                        stat: enums::Stats,
                        dv: u8,
                        level: u16,
                        nature: natures::Nature)
                        -> u16 {
            let mut nature_modifier = 1.0;
            if nature.get_stats().0 == stat {
                nature_modifier = nature_modifier - 0.1;
            }
            if nature.get_stats().1 == stat {
                nature_modifier = nature_modifier + 0.1;
            }
            ((((2.0 * base as f32 + (dv as f32 / 4.0)) * level as f32) / 100.0 + 5.0) *
             nature_modifier) as u16
        }
        println!("{:?}", hp);

        Stats {
            hp: hp,
            attack: {
                stat_formula(model.get_stats().get_stat(&enums::Stats::Attack),
                             enums::Stats::Attack,
                             dv.get_dv(enums::Stats::Attack),
                             level,
                             nature.clone())
            },
            defense: {
                stat_formula(model.get_stats().get_stat(&enums::Stats::Defense),
                             enums::Stats::Defense,
                             dv.get_dv(enums::Stats::Defense),
                             level,
                             nature.clone())
            },
            speed: {
                stat_formula(model.get_stats().get_stat(&enums::Stats::Speed),
                             enums::Stats::Speed,
                             dv.get_dv(enums::Stats::Speed),
                             level,
                             nature.clone())
            },
            special_attack: {
                stat_formula(model.get_stats().get_stat(&enums::Stats::SpecialAttack),
                             enums::Stats::SpecialAttack,
                             dv.get_dv(enums::Stats::SpecialAttack),
                             level,
                             nature.clone())
            },
            special_defense: {
                stat_formula(model.get_stats().get_stat(&enums::Stats::SpecialDefense),
                             enums::Stats::SpecialDefense,
                             dv.get_dv(enums::Stats::SpecialDefense),
                             level,
                             nature)
            },
            accuracy: 100,
            evasion: 100,
        }
    }
    // Getter methods
    //
    /// Gets the stat value of the given stat
    pub fn get_stat(&self, stat: &enums::Stats) -> u16 {
        match stat {
            &enums::Stats::Hp => self.hp,
            &enums::Stats::Attack => self.attack,
            &enums::Stats::Defense => self.defense,
            &enums::Stats::SpecialAttack => self.special_attack,
            &enums::Stats::SpecialDefense => self.special_defense,
            &enums::Stats::Speed => self.speed,
            &enums::Stats::Evasion => self.evasion,
            &enums::Stats::Accuracy => self.accuracy,
            _ => 0,
        }
    }
    // Setter methods
    //
    /// Sets the stat value of the given stat
    pub fn set_stats(&mut self, stat: enums::Stats, value: u16) {
        match stat {
            enums::Stats::Hp => self.hp = value,
            enums::Stats::Attack => self.attack = value,
            enums::Stats::Defense => self.defense = value,
            enums::Stats::SpecialAttack => self.special_attack = value,
            enums::Stats::SpecialDefense => self.special_defense = value,
            enums::Stats::Speed => self.speed = value,
            enums::Stats::Evasion => self.evasion = value,
            enums::Stats::Accuracy => self.accuracy = value,
            _ => {}
        }
    }

    // Provides a default Stat struct with all Values set to 0
    pub fn default() -> Stats {
        Stats {
            hp: 0,
            attack: 0,
            defense: 0,
            special_attack: 0,
            special_defense: 0,
            speed: 0,
            accuracy: 0,
            evasion: 0,
        }
    }
}
