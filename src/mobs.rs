use crate::spatial::Pos;
use crate::tools::math::normalize;
use crate::player::Player;

#[derive(Debug)]
pub enum MoveCategory {
    Terrestrial,
    Aerian,
    Aquatic,
}

pub struct Mob {
    category: MoveCategory,
    pub pos: Pos,
    speed: f32,
    hp: i32,
    armor: i32, // Armor value [0, 100]
    precision: f32, // Chance of hitting the target
    damage: u32, // Base damage
    crit_proba: f32, // Critical hit probability
    crit_mult_dam: f32, // Critical multiplicative damage
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
            crit_mult_dam: 1.5,
            dodge_proba: 0.03,
            in_alert: false,
            is_attacking: false,
            is_alive: true,
        }
    }

    /// Moves a Mob to a x, y coordinates
    pub fn move_to(&mut self, x:i32, y:i32) {
        self.pos.move_to(x, y);
    }

    /// Print some infos about the Mob
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

    /// Euclidian distance between the Mob and the player
    pub fn dist(&self, player_pos:&Player) -> f32 {
        let mob_pos = &self.pos;
        let player_position = &player_pos.pos;
        let distance = mob_pos.dist(player_position);
        return distance;
    }
}