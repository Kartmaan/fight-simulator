//! Set of tools useful for manipulation of numeric and 
//! alphabetic values

/// Mathematical tools
pub mod math {
    use rand::Rng;

    /// Rounds a floating-point number to a given number of 
    /// decimal places.
    /// 
    /// This function takes as input a floating-point 
    /// number `f_num` and an integer `frac_len` 
    /// representing the number of decimal places to be 
    /// retained. It returns the number rounded to the 
    /// nearest decimal place.
    /// 
    /// # Algorithm
    /// 1. Multiply the number by 10 to the power of the 
    /// number of decimal places to be retained.
    /// 2. Round the result to the nearest integer.
    /// 3. Divide the result by 10 raised to the power of the 
    /// number of decimal places to be retained.
    /// 
    /// # Args
    /// * `f_num`: The floating-point number to be rounded.
    /// * `frac_len`: The number of decimal places to retain.
    /// 
    /// # Return
    /// The number `f_num` rounded to `frac_len` decimal places.
    /// (f32)
    /// 
    /// # Example
    /// ```
    /// let x: f32 = 3.141592;
    /// let y: f32 = round(x, 2); // y = 3.14
    /// ```
    pub fn round(f_num: f32, frac_len: u32) -> f32 {
        let multiplier = 10u32.pow(frac_len) as f32;
        (f_num * multiplier).round() / multiplier
    }

    /// Tests a probability based on a normalized value : 
    /// if the probability is realized then the function 
    /// returns `Ok(true)`, otherwise `Ok(false)`.
    /// 
    /// # Args
    /// * `proba` : The probability between 0 and 1 (f32)
    /// 
    /// # Returns
    /// * `Ok(true)` : The probability has been realized
    /// * `Ok(false)` : The probability was not realized
    /// * `Err(String)` : An error has been encountered
    /// 
    /// # Error
    /// Inserting a negative value generates a `panic!`
    /// 
    /// # Example
    /// The function acts like a dice roll. For example, 
    /// if we want an event to occur only once out of 
    /// three:
    /// ```
    /// if check_proba(0.33).unwrap() {
    ///     println!("OK");
    /// } else {
    ///     println!("NOPE");
    /// }
    /// ```
    pub fn check_proba(proba: f32) -> Result<bool, String> {
        let mut proba_val: f32 = proba;
 
        // Values ​​less than or equal to 0 are prohibited.
        if proba < 0.0 {
            return Err(String::from("Value can't be less than zero"));
        
        // Normalization : Perhaps the user tries to enter a 
        // percentage value
        } else if proba > 1.0 {
            proba_val = normalize(proba).unwrap();
        }

        // At this point, we should be sure to have a 'proba_val' 
        // between 0.0 and 1.0

        // Generation of a float between 0 and 1
        let rng_num: f32 = rand::thread_rng().gen();

        // Probability check
        if rng_num < proba_val {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    /// Calculates an exponential reduction of an initial 
    /// value based on a given factor.
    /// 
    /// # Arguments
    /// * `init_value` - The initial value to reduce (f32).
    /// * `factor` - The decline factor that influences 
    /// the intensity of the reduction (f32).
    /// * `k` - Parameter controlling the decay rate.
    /// 
    /// # Returns
    /// * The reduced value after applying the exponential 
    /// reduction (f32).
    /// 
    /// # Example
    /// Let's imagine a damage reduction function: 
    /// `input_value` would be the initial damage, 
    /// while `factor` would be our armor points, the 
    /// higher these are, the greater the reduction of 
    /// `input_value` will be. If the damage received 
    /// is 50 and our armor is 100 with a parameter k 
    /// set to 0.0217 then the function will be called 
    /// as follows: 
    /// ```
    /// let final_dam = exp_decay(50.0, 100.0, 0.0217);
    /// println!("{}", final_dam) // 5.708
    /// ```
    /// The 50 initial damage is reduced to around 5.7, 
    /// the higher the armor value the more effective 
    /// this defense will be and vice versa.
    /// 
    /// # Note
    /// *This function uses the exponential function 
    /// `exp()` from the Rust standard library whose 
    /// precision is not deterministic*.
    pub fn exp_decay(input_value: f32, factor: f32, k: f32) -> f32 {
        let final_dam: f32 = input_value * (-k * factor).exp();
        return  final_dam;
    }

    /// Normalizes a value to be between 0 and 1.
    /// 
    /// # Details
    /// The function will attempt by several means to 
    /// normalize the value according to its order of 
    /// magnitude.
    /// - If the value is within the range [0,1] it's 
    /// returned as is. 
    /// - If the value is within the range ]1,100] then 
    /// the it's divided by 100.
    /// - All values ​​greater than 100 become 1.0.
    /// - Otherwise the function returns an error (we assume 
    /// that the value is negative).
    /// 
    /// # Args
    /// * `value` - The f32 value to be normalized.
    /// 
    /// # Returns
    /// * `Ok(f32)` - The normalized value if valid.
    /// * `Err(String)` - Error message if the value is 
    /// invalid.
    /// 
    /// # Examples
    /// ```
    /// assert_eq!(normalize(0.5).unwrap(), 0.5);
    /// assert_eq!(normalize(50.0).unwrap(), 0.5);
    /// assert_eq!(normalize(150.0).unwrap(), 1.0);
    /// assert!(normalize(-1.0).is_err());
    /// ```
    pub fn normalize(value: f32) -> Result<f32, String> {
        match value {
            v if v >= 0.0 && v <= 1.0 => Ok(v),
            v if v > 1.0 && v <= 100.0 => Ok(v / 100.0),
            v if v > 100.0 => Ok(1.0),
            _ => Err(String::from("Speed value must be between 0 and 1")),
        }
    }

    /// Generates a random value centered around a given 
    /// value.
    /// 
    /// The range limits are plus and minus 1/`fraction` 
    /// of the central value.
    /// 
    /// # Args
    /// * 'central_value' : The value around which to 
    /// center the random number
    /// * 'fraction' : Fraction of 'central_value' which 
    /// will be the half range around it (see exemple).
    /// 
    /// # Return
    /// An integer random number between the range
    /// 
    /// # Example
    /// * `central_value` = 10
    /// * `fraction` = 2 \
    /// The width of the range centered on `central_value`
    /// will be `central_value` / `fraction` = 5. The 
    /// random value will therefore oscillate between 5 
    /// and 15. The smaller the `fraction` value, the 
    /// wider the oscillation. 
    pub fn centred_rand(central_value: f32, fraction: f32) -> f32 {
        let mut half_range = central_value / fraction;
        if half_range < 1.0 {
            half_range = half_range.ceil();
        }

        let from = (central_value - half_range);
        let to = (central_value + half_range);
        let rand_val = rand::thread_rng().gen_range(from..=to);

        rand_val
    }
}

/// Structures and methods for geometric operations in 
/// 2D space
pub mod spatial {
    /// 2D coordinates structure
    #[derive(Debug, Clone, Default)]
    pub struct Pos {
        pub x: i32,
        pub y: i32,
    }
    
    impl Pos {
        /// Create a new Pos struct
        pub fn new(x: i32, y:i32) -> Pos {
            Pos {x:x, y:y}
        }
    
        /// Change the coordinates of a Pos struct
        pub fn move_to(&mut self, x:i32, y:i32) {
            self.x = x;
            self.y = y;
        }
    
        /// Euclidian distance between two coordinates
        pub fn dist(&self, other:&Pos) -> f32 {
            let res = 
            ((other.x - self.x).pow(2) as f32) + 
            ((other.y - self.y).pow(2) as f32);
            res.sqrt()
        }
    }
}

/// Functions defining some game mechanics
pub mod game_mechanics {
    use color_print::cprintln;

    use super::traits::Mortal;
    use super::math::{check_proba, exp_decay, centred_rand};

    /// Returns the effective damage of a `Mortal`.
    /// 
    /// The final damage can vary depending on several 
    /// parameters such as the `precision`, `damage` and 
    /// `damage_variation` value of `attacker`.
    /// 
    /// # Args
    /// * `attacker`: Bearer of the `Mortal` trait. can 
    /// be a `Mob` or a `Player` 
    /// 
    /// # Return
    /// * The final damage of `attacker` (`f32`).
    pub fn attack<T: Mortal>(attacker: &T) -> f32 {
        // The accuracy test is passed : the blow is delivered
        if check_proba(attacker.get_precision()).unwrap() {
            let base_dam: f32 =  centred_rand(
                attacker.get_damage(),
                attacker.get_damage_variation());
            let mut base_dam: f32 = base_dam as f32;

            // Crit realized
            if check_proba(attacker.get_crit_proba()).unwrap() {
                cprintln!("<red>CRIT by {} !</red>", attacker.get_name());
                base_dam = base_dam * attacker.get_crit_multiplier();
                base_dam

            // No crit
            } else {
                base_dam
            }

        // Missed hit
        } else {
            cprintln!("<yellow>MISSED by {} !</yellow>", attacker.get_name());
            let base_dam: f32= 0.0;
            base_dam
        }
    }

    /// A `Mortal` takes a damage.
    /// 
    /// `defender` armor and/or HP values ​​are directly 
    /// modified according to several parameters such as 
    /// `defender`s armor and `dodge_proba` value.
    /// 
    /// # Args
    /// * `defender` : The one who receives the damage. 
    /// Can be a `Mob` or a `Player`.
    /// * `damage` : The amount of damage received.
    pub fn defense<T: Mortal>(defender: &mut T, damage: f32) {
        // No dodging - Right in the face
        if !check_proba(defender.get_dodge_proba()).unwrap() {
            // Armor is present
            if defender.get_armor() > 0.0 {
                let dam: f32 = damage;
                let armor: f32 = defender.get_armor();
                let k: f32 = defender.get_armor_decay_rate();

                let final_dam: f32 = exp_decay(
                    dam, 
                    armor as f32, 
                    k);

                // Armor will be able to absorb the damage
                if final_dam < armor as f32 {
                    defender.set_armor(armor - final_dam);
                
                // Armor can only take a fraction of the 
                //damage
                } else {
                    let hp: i32 = defender.get_hp();
                    let extra_dam: f32 = final_dam - armor;
                    defender.set_armor(0.0);
                    defender.set_hp(hp - extra_dam as i32);
                }
            
            // Armor is broken
            } else {
                // Still alive
                if defender.get_hp() > 0 {
                    // HP points can take the damage
                    if damage < defender.get_hp() as f32 {
                        defender.set_hp(defender.get_hp() - damage as i32);
                    
                    // HP points can't absorb the damage
                    } else {
                        defender.kill();
                    }
                    
                
                // Already dead, but just in case...
                } else {
                    defender.kill();
                }
            }
        // Dodge
        } else {
            cprintln!("<green>DODGED by {} !</green>", defender.get_name());
        }
    }

    /// Let them fight : Fight between two `Mortal`s
    /// 
    /// Two `Mortal` trait holders exchange blows until 
    /// one of them has no HP left.
    pub fn battle<T: Mortal, U: Mortal>(fighter_1: &mut T, fighter_2: &mut U) {
        let mut damage: f32;

        // It's a bit creepy to say, but the only way out 
        // of this loop is for one of the two fighters 
        // to die.
        loop {
            // figher_1 attacks fighter_2
            damage = attack(fighter_1);

            println!("{} attacks {} : {} dam", 
            fighter_1.get_name(), fighter_2.get_name(),
            &damage);

            defense(fighter_2, damage);
            println!("{} -> Armor : {} | HP : {}",
            fighter_2.get_name(), 
            fighter_2.get_armor(), 
            fighter_2.get_hp());

            println!("________________");

            // fighter_2 still alive and counter attacking
            if fighter_2.get_hp() > 0 {
                damage = attack(fighter_2);

                println!("{} attacks {} : {} dam", 
                fighter_2.get_name(), fighter_1.get_name(),
                &damage);

                defense(fighter_1, damage);
                println!("{} -> Armor : {} | HP : {}",
                fighter_1.get_name(),
                fighter_1.get_armor(), 
                fighter_1.get_hp());

            // fighter_2 dies -> figher_1 wins
            } else {
                println!("{} wins", fighter_1.get_name());
                break;
            }

            println!("________________");

            // fighter_1 resisted the blow
            if fighter_1.get_hp() > 0 {
                continue;
            // fighter_1 dies -> figher_2 wins
            } else {
                println!("{} wins", fighter_2.get_name());
                break;
            }
        }
    }
}

/// Module containing all the traits useful for this project
pub mod traits {
    use super::spatial::Pos;
    /// Anything that can attack, defend and die.
    pub trait Mortal {
        // ----- Gets -----
        fn get_name(&self) -> String;
        fn get_hp(&self) -> i32;
        fn get_armor(&self) -> f32;
        fn get_armor_decay_rate(&self) -> f32;
        fn get_precision(&self) -> f32;
        fn get_damage(&self) -> f32;
        fn get_damage_variation(&self) -> f32;
        fn get_crit_proba(&self) -> f32;
        fn get_crit_multiplier(&self) -> f32;
        fn get_dodge_proba(&self) -> f32;
        fn get_in_alert(&self) -> bool;
        fn get_is_attacking(&self) -> bool;
        fn get_is_alive(&self) -> bool;

        //  ----- Sets -----
        fn set_hp(&mut self, new_hp: i32);
        fn set_armor(&mut self, new_armor: f32);
        fn set_in_alert(&mut self, new_bool: bool);
        fn set_is_attacking(&mut self, new_bool: bool);
        fn set_is_alive(&mut self, new_bool: bool);

        //  ----- Actions -----
        /// Gives full meaning to the Mortal trait
        fn kill(&mut self);
    }

    /// Everything that can be located in space
    pub trait Located {
        /// Returns the position of a Located trait carrier
        fn get_pos(&self) -> Pos;

        /// Returns the euclidean distance between two 
        /// carriers of the Located trait
        fn get_distance<T: Located>(&self, other: &T) -> f32;

        /// Changes the position of a Located trait carrier
        fn set_pos(&mut self, new_pos: Pos);
    }
}