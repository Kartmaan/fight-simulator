//! Set of tools useful for manipulation of numeric and 
//! alphabetic values

//use crate::tools::traits::Entity;

/// Mathematical tools
pub mod math {
    use rand::Rng;

    /// Rounds a floating-point number to a given number of 
    /// decimal places.
    /// 
    /// This function takes as input a floating-point number 
    /// `f_num` and an integer `frac_len` representing the number 
    /// of decimal places to be retained. It returns the number 
    /// rounded to the nearest decimal place.
    /// 
    /// **Algorithm**
    /// 1. Multiply the number by 10 to the power of the number 
    /// of decimal places to be retained.
    /// 2. Round the result to the nearest integer.
    /// 3. Divide the result by 10 raised to the power of the 
    /// number of decimal places to be retained.
    /// 
    /// **Args**
    /// * `f_num`: The floating-point number to be rounded.
    /// * `frac_len`: The number of decimal places to retain.
    /// 
    /// **Return**
    /// The number `f_num` rounded to `frac_len` decimal places.
    /// 
    /// **Example**
    /// ```rust
    /// let x: f32 = 3.141592;
    /// let y: f32 = round(x, 2); // y = 3.14
    /// ```
    pub fn round(f_num: f32, frac_len: u32) -> f32 {
        let multiplier = 10u32.pow(frac_len) as f32;
        (f_num * multiplier).round() / multiplier
    }

    /// Tests a probability based on a normalized value : if the 
    /// probability is realized then the function returns `true`, 
    /// otherwise `false`.
    /// 
    /// # Arguments
    /// 
    /// * `proba` : The probability between 0 and 1 (f32)
    /// 
    /// # Returns
    /// 
    /// * `Ok(true)` : The probability has been realized
    /// * `Ok(false)` : The probability was not realized
    /// * `Err(String)` : An error has been encountered
    /// 
    /// # Example
    /// 
    /// The function acts like a dice roll. For example, if we want 
    /// an event to occur only once out of three:
    /// ```
    /// if check_proba(0.33).unwrap() {
    ///     println!("OK");
    /// } else {
    ///     println!("NOPE");
    /// }
    /// ```
    pub fn check_proba(proba: f32) -> Result<bool, String> {
        //let coef: f32;
        
        // A probability of 0 could create infinite loops. 
        // Values ​​less than or equal to 0 are prohibited.
        if proba <= 0.0 {
            return Err(String::from("Value can't be less than zero"));
        }

        // Generation of a float between 0 and 1
        let rng_num: f32 = rand::thread_rng().gen();

        // Probability check
        if rng_num < proba {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    /// Calculates an exponential reduction of an initial value based 
    /// on a given factor.
    /// 
    /// # Arguments
    /// 
    /// * `init_value` - The initial value to reduce (f32).
    /// * `factor` - The decline factor that influences the intensity 
    /// of the reduction (f32).
    /// * `k` - Parameter controlling the decay rate.
    /// 
    /// # Returns
    /// 
    /// * The reduced value after applying the exponential 
    /// reduction (f32).
    /// 
    /// # Example
    /// Let's imagine a damage reduction function: `input_value` 
    /// would be the initial damage, while `factor` would be our 
    /// armor points, the higher these are, the greater the reduction 
    /// of `input_value` will be. If the damage received is 50 and 
    /// our armor is 100 with a parameter k set to 0.0217 then the 
    /// function will be called as follows: 
    /// ```
    /// let final_dam = exp_decay(50.0, 100.0, 0.0217);
    /// println!("{}", final_dam) // 5.708
    /// ```
    /// The 50 initial damage is reduced to around 5.7, the higher 
    /// the armor value the more effective this defense will be 
    /// and vice versa.
    /// 
    /// # Note
    /// *This function uses the exponential function `exp()` from the 
    /// Rust standard library whose precision is not deterministic*.
    pub fn exp_decay(input_value: f32, factor: f32, k: f32) -> f32 {
        let final_dam: f32 = input_value * (-k * factor).exp();
        return  final_dam;
    }

    /// Normalizes a value to be between 0 and 1.
    /// 
    /// # Details
    /// The function will attempt by several means to normalize the 
    /// value according to its order of magnitude.
    /// - If the value is within the range [0,1] it's returned as is. 
    /// - If the value is within the range ]1,100] then the it's 
    /// divided by 100.
    /// - All values ​​greater than 100 become 1.0.
    /// - Otherwise the function returns an error (we assume that 
    /// the value is negative).
    /// 
    /// # Arguments
    /// * `value` - The f32 value to be normalized.
    /// 
    /// # Return value
    /// * `Ok(f32)` - The normalized value if valid.
    /// * `Err(String)` - Error message if the value is invalid.
    /// 
    /// # Exemples
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

    /// Generates a random value centered around a given value
    /// 
    /// The range limits are plus and minus 1/8 of the 
    /// central value.
    /// 
    /// **Args**
    /// * 'central_value' : The value around which to center the 
    /// random number
    /// 
    /// **Return**
    /// An integer random number between the range
    pub fn centred_rand(central_value: u32, fraction: f32) -> u32 {
        //let fraction: f32 = 8.0; // Fraction of 'central_value'
        let central = central_value as f32;
        let mut half_range = central / fraction;
        if half_range < 1.0 {
            half_range = half_range.ceil();
        }

        let from = (central - half_range) as u32;
        let to = (central + half_range) as u32;
        let rand_val = rand::thread_rng().gen_range(from..=to);

        rand_val
    }
}

/// Structures and methods for geometric operations in 2D space
pub mod spatial {
    /// 2D coordinates structure
    #[derive(Debug, Clone)]
    pub struct Pos {
        pub x: i32,
        pub y: i32,
    }
    
    impl Pos {
        /// Create a new Pos struct
        pub fn new(x: i32, y:i32) -> Pos {
            Pos {x:x, y:y}
        }
    
        /// Initializes default coordinates
        pub fn default() -> Pos {
            Pos {x:0, y:0}
        }
    
        /// Change the coordinates of a Pos struct
        pub fn move_to(&mut self, x:i32, y:i32) {
            self.x = x;
            self.y = y;
        }
    
        /// Euclidian distance between two coordinates
        pub fn dist(&self, other:&Pos) -> f32 {
            let res = (other.x - self.x).pow(2) + (other.y - self.y).pow(2);
            (res as f32).sqrt()
        }
    }
}

/// Functions useful for game mechanics
pub mod game_mechanics {
    use super::traits::Mortal;
    use super::math::{round, check_proba, exp_decay, centred_rand};

    /// A Mortal attacks another Mortal
    /// 
    /// 'attacker' can be a Mob or Player type.
    pub fn attack<T: Mortal>(attacker: &T) -> u32 {
        // Successful hit
        if check_proba(attacker.get_precision()).unwrap() {
            let base_dam: u32 =  centred_rand(
                attacker.get_damage(),
                attacker.get_damage_variation());
            let mut base_dam: f32 = base_dam as f32;

            // Crit realized
            if check_proba(attacker.get_crit_proba()).unwrap() {
                base_dam = base_dam * attacker.get_crit_multiplier();
                base_dam as u32

            // No crit
            } else {
                base_dam as u32
            }

        // Missed hit
        } else {
            let base_dam: u32 = 0;
            base_dam
        }
    }

    pub fn defense<T: Mortal>(defender: &mut T, damage: u32) {
        // No dodge
        if !check_proba(defender.get_dodge_proba()).unwrap() {
            // Armor is present
            if defender.get_armor() > 0 {
                let dam: f32 = damage as f32;
                let armor: i32 = defender.get_armor();
                let k: f32 = 0.04;

                let final_dam: f32 = exp_decay(
                    dam, 
                    armor as f32, 
                    k);

                // Armor will be able to absorb the damage
                if final_dam < armor as f32 {
                    defender.set_armor(armor - final_dam as i32);
                
                // Armor can only take a fraction of the damage
                } else {
                    let hp: i32 = defender.get_hp();
                    let extra_dam: f32 = final_dam - armor as f32;
                    defender.set_armor(0);
                    defender.set_hp(hp - extra_dam as i32);
                }
            
            // Armor is broken
            } else {
                // Still alive
                if defender.get_hp() > 0 {
                    defender.set_hp(defender.get_hp() - damage as i32);
                
                // Already dead...
                } else {
                    defender.set_hp(0);
                }
            }
        println!("Armor: {} | HP: {} ", defender.get_armor(), defender.get_hp());
        // Dodge
        } else {
            println!("DODGE !");
        }
    }
}

pub mod traits {
    /// Bearers of this trait can attack, take damage and 
    /// eventually (fatally) die.
    pub trait Mortal {
        // Gets
        fn get_hp(&self) -> i32;
        fn get_armor(&self) -> i32;
        fn get_precision(&self) -> f32;
        fn get_damage(&self) -> u32;
        fn get_damage_variation(&self) -> f32;
        fn get_crit_proba(&self) -> f32;
        fn get_crit_multiplier(&self) -> f32;
        fn get_dodge_proba(&self) -> f32;

        // Sets
        fn set_hp(&mut self, new_hp: i32);
        fn set_armor(&mut self, new_armor: i32);
    }
}