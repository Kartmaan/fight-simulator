//! Module defining the Mob structure and all its implementations
//! as well as the BESTIARY

use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::tools::spatial::Pos;
use crate::tools::math::{normalize, exp_decay};
use crate::tools::traits::Mortal;
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
            speed: normalize(15.0).unwrap(),
            hp: 230,
            armor: 0,
            precision: 0.95,
            damage: 40,
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
            speed: normalize(9.0).unwrap(),
            hp: 80,
            armor: 50,
            precision: 0.95,
            damage: 20,
            crit_proba: 0.1,
            crit_multiplier: 2.0,
            dodge_proba: 0.05,
            in_alert: false,
            is_attacking: false,
            is_alive: true,
        });

        // SHARK
        map.insert("shark", Mob {
            category: MoveCategory::Aquatic,
            pos: Pos::default(),
            speed: normalize(9.0).unwrap(),
            hp: 70,
            armor: 0,
            precision: 0.75,
            damage: 40,
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
    armor: i32, // Armor value [0, 100]
    precision: f32, // Chance of hitting the target
    damage: u32, // Base damage
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
            speed: normalize(speed).unwrap(),
            hp: 100,
            armor: 40,
            precision: 0.9,
            damage: 33,
            crit_proba: 0.05,
            crit_multiplier: 1.5,
            dodge_proba: 0.03,
            in_alert: false,
            is_attacking: false,
            is_alive: true,
        }
    }

    /// Returns the position of the Mob
    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }

    /// Moves a Mob to a x, y coordinates
    pub fn move_to(&mut self, x:i32, y:i32) {
        self.pos.move_to(x, y);
    }

    /// Prints some infos about the Mob
    pub fn info(&self) {
        println!("\nCategory : {:?}", self.category);
        println!("Speed : {}", self.speed);
        println!("Pos x,y : ({},{})", self.pos.x, self.pos.y);
        println!("HP : {}", self.hp);
        println!("Alive : {}", self.is_alive);
    }

    /// The Mob recieved damage
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

    fn get_armor(&self) -> i32 {
        self.armor
    }

    fn get_precision(&self) -> f32 {
        self.precision
    }

    fn get_damage(&self) -> u32 {
        self.damage
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

    // ------ SETS ------
    fn set_hp(&mut self, new_hp: i32) {
        self.hp = new_hp;
    }

    fn set_armor(&mut self, new_armor: i32) {
        self.armor = new_armor;
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