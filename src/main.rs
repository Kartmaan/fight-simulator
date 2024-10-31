use std::io;

#[derive(Debug)]
enum MoveCategory {
    Terrestrial,
    Aerian,
    Aquatic,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

struct Mob {
    name: String,
    category: MoveCategory,
    pos: Pos,
    speed: f32,
    hp: i32,
    in_alert: bool,
    is_attacking: bool,
    alive: bool,
}

fn speed_check(speed: f32) -> Result<f32, String> {
    if speed >= 0.0 && speed <= 1.0 {
        Ok(speed)
    } else {
        Err(String::from("Speed value must be between 0 and 1"))
    }
}

impl Mob {
    fn new(name: &str, cat: MoveCategory, speed: f32) -> Mob {
        //let speed = 

        Mob {
            name: name.to_string(),
            category: cat,
            pos: Pos{x:0, y:0},
            speed: speed_check(speed).unwrap(),
            hp: 100,
            in_alert: false,
            is_attacking: false,
            alive: true,
        }
    }

    fn info(&self) {
        println!("\nMob name : {}", self.name);
        println!("Category : {:?}", self.category);
        println!("Speed : {}", self.speed);
        println!("Pos x/y : {}/{}", self.pos.x, self.pos.y);
        println!("HP : {}", self.hp);
        println!("Alive : {}", self.alive);
    }

    fn hit(&mut self, damage: i32) {
        self.hp = self.hp - damage;

        // Damage reducing health below 0
        if self.hp < 0 {
            self.hp = 0;
        }

        if self.hp > 0 {
            self.alive = true;
        } else {
            self.alive = false;
        }
    }
}

fn main() {
    let mut dragon = Mob::new("Fangorn", MoveCategory::Aerian, 0.33);

    dragon.info();

    dragon.hit(50);

    dragon.info();

    dragon.hit(50);

    dragon.info();
}