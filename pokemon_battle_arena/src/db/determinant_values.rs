extern crate rand;

use self::rand::{Rng, thread_rng};
use super::enums::Stats;

/// Struct for the Determinant values of a Pokemon. Can have values between 0 and 31 and are
/// randomly provided. The influence the base stats of the PokemonToken.
#[derive(Debug, Clone)]
pub struct Dv {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed: u8,
}

impl Dv {
    /// Randomly provides 6 values between 0 and 31. For legendary Pokemon 3 random stats are set to
    /// 31 automatically. Legendaries currently unimplemented.
    pub fn get_dvs() -> Dv {
        let mut rng = thread_rng();
        let mut sample = Vec::new();
        let count = 0;
        // if poke.is_legendary {
        //     for _ in 0..3 {
        //         sample.push(31);
        //     }
        //     count = 2;
        // }
        for _ in count..6 {
            sample.push(rng.gen_range(0, 32))
        }
        rng.shuffle(&mut sample);
        Dv {
            hp: sample[0],
            attack: sample[1],
            defense: sample[2],
            special_attack: sample[3],
            special_defense: sample[4],
            speed: sample[5],
        }
    }
    /// Returns the determinant value given in the parameters
    pub fn get_dv(&self, stat: Stats) -> u8 {
        match stat {
            Stats::Hp => self.hp,
            Stats::Attack => self.attack,
            Stats::Defense => self.defense,
            Stats::SpecialAttack => self.special_attack,
            Stats::SpecialDefense => self.special_attack,
            Stats::Speed => self.speed,
            _ => 1,
        }
    }
}
