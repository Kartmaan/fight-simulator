//! Module defining the Mob structure and all its 
//! implementations as well as the BESTIARY

//use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::utils::spatial::Pos;
use crate::utils::traits::{Mortal, Located};
use crate::player::Player;

/// The different types of movement that a Mob can adopt
#[derive(Debug, Clone)]
pub enum MoveCategory {
    Terrestrial,
    Aerian,
    Aquatic,
}

lazy_static::lazy_static! {
    /// Bestiary containing different types of Mob
    pub static ref BESTIARY: HashMap<&'static str, Mob> = {
        let mut map = HashMap::new();

        // DRAGON
        map.insert("dragon", Mob {
            category: MoveCategory::Aerian,
            pos: Pos::default(),
            speed: 0.25,
            hp: 230,
            armor: 0.0,
            precision: 0.95,
            damage: 40.0,
            damage_variation: 8.0,
            crit_proba: 0.1,
            crit_multiplier: 2.0,
            dodge_proba: 0.05,
            in_alert: false,
            is_attacking: false,
            is_alive: true,
        });

        // GOBELIN
        map.insert("gobelin", Mob {
            category: MoveCategory::Terrestrial,
            pos: Pos::default(),
            speed: 0.25,
            hp: 100,
            armor: 100.0,
            precision: 0.95,
            damage: 45.0,
            damage_variation: 8.0,
            crit_proba: 0.1,
            crit_multiplier: 2.0,
            dodge_proba: 0.15,
            in_alert: false,
            is_attacking: false,
            is_alive: true,
        });

        // SHARK
        map.insert("shark", Mob {
            category: MoveCategory::Aquatic,
            pos: Pos::default(),
            speed: 0.25,
            hp: 70,
            armor: 0.0,
            precision: 0.85,
            damage: 40.0,
            damage_variation: 8.0,
            crit_proba: 0.1,
            crit_multiplier: 2.0,
            dodge_proba: 0.05,
            in_alert: false,
            is_attacking: false,
            is_alive: true,
        });
        map
    };
}

/// Player's enemy
#[derive(Debug, Clone)]
pub struct Mob {
    category: MoveCategory,
    pos: Pos,
    speed: f32,
    hp: i32,
    armor: f32, // Armor value [0, 100]
    precision: f32, // Chance of hitting the target
    damage: f32, // Base damage
    damage_variation: f32,
    crit_proba: f32, // Critical hit probability
    crit_multiplier: f32, // Critical multiplicative damage
    dodge_proba: f32, // Probability to dodge a hit
    in_alert: bool, // Mob's looking for trouble
    is_attacking: bool, // Mob's under attack
    is_alive: bool, // Mob's still alive
}

impl Mob {
    /// Create a new Mob
    pub fn new(cat: MoveCategory, speed: f32, pos: Pos) -> Mob {
        Mob {
            category: cat,
            pos: pos,
            speed: 0.25,
            hp: 100,
            armor: 40.0,
            precision: 0.9,
            damage: 33.0,
            damage_variation: 8.0,
            crit_proba: 0.05,
            crit_multiplier: 1.5,
            dodge_proba: 0.03,
            in_alert: false,
            is_attacking: false,
            is_alive: true,
        }
    }

    /// Prints some infos about the Mob
    pub fn info(&self) {
        println!("\nCategory : {:?}", self.category);
        println!("Speed : {}", self.speed);
        println!("Pos x,y : ({},{})", self.pos.x, self.pos.y);
        println!("Armor : {}", self.armor);
        println!("HP : {}", self.hp);
        println!("Alive : {}", self.is_alive);
    }

    /// Euclidian distance between a Mob and the player
    pub fn dist(&self, player_pos:&Player) -> f32 {
        let mob_pos = &self.pos;
        let player_position = &player_pos.pos;
        let distance = mob_pos.dist(player_position);
        return distance;
    }

    /// Kills a Mob in cold blood
    pub fn kill(&mut self) {
        self.hp = 0;
        self.in_alert = false;
        self.is_attacking = false;
        self.is_alive = false;
    }
}

impl Mortal for Mob {
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

impl Located for Mob {
    fn get_pos(&self) -> Pos {
        self.pos.clone()
    }

    fn get_distance<T: Located>(&self, other: &T) -> f32 {
        let mob_pos = self.pos.clone();
        let other_pos = other.get_pos();
        mob_pos.dist(&other_pos)
    }

    fn set_pos(&mut self, new_pos: Pos) {
        self.pos = new_pos;
    }
}

/// Returns Ok(Mob) if the Mob name is available in BESTIARY
pub fn get_mob(mob_name: &str) -> Result<Mob, String> {
    if BESTIARY.contains_key(mob_name) {
        let mut mob: Mob = BESTIARY.get(mob_name).cloned().unwrap();
        Ok(mob)
    } else {
        let err_txt = format!("Mob '{}' not found in bestiary", mob_name);
        Err(String::from(err_txt))
    }
}