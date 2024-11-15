//! Module defining the Player structure and all its 
//! implementations

use crate::utils::spatial::Pos;
use crate::utils::traits::{Mortal, Located};

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
    armor: f32, // Armor value [0, 100]
    precision: f32, // Chance of hitting the target
    damage: f32, // Base damage
    damage_variation: f32, // damage fraction
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
                    hp: 100,
                    armor: 100.0,
                    precision: 0.9,
                    damage: 45.0,
                    damage_variation: 8.0,
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
                    armor: 60.0,
                    precision: 0.75,
                    damage: 50.0,
                    damage_variation: 8.0,
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

    /// Prints Player's infos
    pub fn info(&self) {
        println!("\nName : {:?}", self.name);
        println!("\nClass : {:?}", self.class);
        println!("Speed : {}", self.speed);
        println!("Pos x,y : ({},{})", self.pos.x, self.pos.y);
        println!("Armor : {}", self.armor);
        println!("HP : {}", self.hp);
        println!("Alive : {}", self.is_alive);
    }
}

impl Mortal for Player {
    // ------ GETS ------
    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn get_armor(&self) -> f32 {
        self.armor
    }

    fn get_precision(&self) -> f32 {
        self.precision
    }

    fn get_damage(&self) -> f32 {
        self.damage
    }

    fn get_damage_variation(&self) -> f32 {
        self.damage_variation
    }

    fn get_crit_proba(&self) -> f32 {
        self.crit_proba
    }

    fn get_crit_multiplier(&self) -> f32 {
        self.crit_multiplier
    }

    fn get_dodge_proba(&self) -> f32 {
        self.dodge_proba
    }

    fn get_in_alert(&self) -> bool {
        self.in_alert
    }

    fn get_is_attacking(&self) -> bool {
        self.is_attacking
    }

    fn get_is_alive(&self) -> bool {
        self.is_alive
    }

    // ------ SETS ------
    fn set_hp(&mut self, new_hp: i32) {
        self.hp = new_hp;
    }

    fn set_armor(&mut self, new_armor: f32) {
        self.armor = new_armor;
    }

    fn set_in_alert(&mut self, new_bool: bool) {
        self.in_alert = new_bool;
    }

    fn set_is_attacking(&mut self, new_bool: bool) {
        self.is_attacking = new_bool;
    }

    fn set_is_alive(&mut self, new_bool: bool) {
        self.is_alive = new_bool;
    }

    // ------ Actions ------
    fn kill(&mut self) {
        self.armor = 0.0;
        self.hp = 0;
        self.in_alert = false;
        self.is_attacking = false;
        self.is_alive = false;
    }
}

impl Located for Player {
    fn get_pos(&self) -> Pos {
        self.pos.clone()
    }

    fn get_distance<T: Located>(&self, other: &T) -> f32 {
        let player_pos = self.pos.clone();
        let other_pos = other.get_pos();
        player_pos.dist(&other_pos)
    }

    fn set_pos(&mut self, new_pos: Pos) {
        self.pos = new_pos;
    }
}