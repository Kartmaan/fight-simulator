//! Module defining the Player structure and all its 
//! implementations

use crate::tools::spatial::Pos;
use crate::tools::math::{normalize, exp_decay};
use crate::tools::traits::Mortal;
use crate::mobs::Mob;

/// The different classes that can be chosen by the player. 
/// They can bring penalties or bonuses to their characteristics.
#[derive(Debug)]
pub enum PlayerClass {
    Archer,
    Warrior,
}

/// The character controlled by the player
pub struct Player {
    name: String,
    class: PlayerClass,
    pub pos: Pos,
    speed: f32,
    hp: i32,
    armor: i32, // Armor value [0, 100]
    precision: f32, // Chance of hitting the target
    damage: u32, // Base damage
    damage_variation: i32, // +/- x%
    crit_proba: f32, // Critical hit probability
    crit_multiplier: f32, // Critical multiplicative damage
    dodge_proba: f32, // Probability to dodge a hit
    in_alert: bool, // Mob's looking for trouble
    is_attacking: bool, // Mob's under attack
    is_alive: bool, // Mob's still alive
}

impl Player {
    /// Creating a new player character
    pub fn new(name: String, class: PlayerClass, pos:Pos) -> Player {
        match class {
            PlayerClass::Warrior => {
                Player {
                    name: name,
                    class: class,
                    pos: pos,
                    speed: 0.25,
                    hp: 200,
                    armor: 100,
                    precision: 0.9,
                    damage: 45,
                    damage_variation: 2,
                    crit_proba: 0.05,
                    crit_multiplier: 2.0,
                    dodge_proba: 0.08,
                    in_alert: false,
                    is_attacking: false,
                    is_alive: true,
                }
            }

            PlayerClass::Archer => {
                Player {
                    name: name,
                    class: class,
                    pos: pos,
                    speed: 0.4,
                    hp: 100,
                    armor: 60,
                    precision: 0.75,
                    damage: 50,
                    damage_variation: 1,
                    crit_proba: 0.05,
                    crit_multiplier: 2.5,
                    dodge_proba: 0.15,
                    in_alert: false,
                    is_attacking: false,
                    is_alive: true,
                }
            }
        } // match
    }

    /// Player movement
    pub fn move_to(&mut self, x:i32, y:i32) {
        self.pos.move_to(x, y);
    }

    /// Prints Player's infos
    pub fn info(&self) {
        println!("\nName : {:?}", self.name);
        println!("\nClass : {:?}", self.class);
        println!("Speed : {}", self.speed);
        println!("Pos x,y : ({},{})", self.pos.x, self.pos.y);
        println!("HP : {}", self.hp);
        println!("Alive : {}", self.is_alive);
    }

    /// Receiving damage
    pub fn hit(&mut self, damage: i32) {
        self.hp = self.hp - damage;

        // Damage reducing health below 0
        if self.hp < 0 {
            self.hp = 0;
        }

        if self.hp > 0 {
            self.is_alive = true;
        } else {
            self.is_alive = false;
        }
    }

    /// Euclidian distance between a Player and a Mob
    pub fn dist(&self, mob_pos:&Mob) -> f32 {
        let player_pos = &self.pos;
        let mob_pos = &mob_pos.get_pos();
        let distance = player_pos.dist(mob_pos);
        return distance;
    }
}

impl Mortal for Player {
    fn get_damage(&self) -> u32 {
        self.damage
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn set_hp(&mut self, new_hp: i32) {
        self.hp = new_hp;
    }
}