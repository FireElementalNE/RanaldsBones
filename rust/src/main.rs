use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use bones::args::{AppArgs, BONUS_ARG, GRIM_ARG, TOME_ARG};


const DICE_COUNT: u64 = 7_u64;
const MAX_ROLL: f64 = 1000_f64;
const NCUTOFF: f64 = (1.0 / 3.0) * MAX_ROLL;
const TCUTOFF: f64 = (2.0 / 3.0) * MAX_ROLL;
const BCUTOFF: f64 = 499_f64;

pub struct Dice {
    rng: ChaCha20Rng,
}

impl Dice {
    pub fn new(seed: u64) -> Dice {
        Dice {
            rng: ChaCha20Rng::seed_from_u64(seed)
        }
    }
    pub fn roll(&mut self, cutoff: u64) -> u64 {
        match (self.rng.gen::<u64>() % 1000) < cutoff {
            true => 1_u64,
            false => 0_u64
        }
    }
}

fn roll_dice_helper(mut total: u64, name: &str, count: u64, cutoff: f64) -> Result<u64> {
    let mut dice = Dice::new(urandom::new().next_u64());
    for i in 0..count {
        let roll_val = dice.roll(cutoff as u64);
        let front_str = match roll_val {
            1 => "> S",
            _ => "> F"
        };
        println!("{} for {} # {}",front_str, name, i+1);
        total = total + roll_val;
    }
    Ok(total)
}

fn roll_dice(tomes: u64, grims: u64, bonus: u64) -> Result<()>{
    let d = DICE_COUNT - tomes - grims - bonus;
    let mut total = 0_u64;
    println!("Dice ppol is: ");
    println!(" {tomes} Tome(s)");
    println!(" {grims} Grimoire(s)");
    println!(" {bonus} Bonus Dice");
    println!(" {d} Regular Dice");
    println!("-------------------");
    total = roll_dice_helper(total,"Tome", tomes, TCUTOFF, )?;
    total = roll_dice_helper(total,"Grimoir", grims, MAX_ROLL, )?;
    total = roll_dice_helper(total, "Bonus Dice", bonus, BCUTOFF)?;
    total = roll_dice_helper(total, "Regular Dice" , d, NCUTOFF)?;
    // println!("-------------------");
    println!("Total is {total}");
    Ok(())

}

fn main() -> Result<()> {
    let args = AppArgs::new();
    let tomes = args.get_argument::<u64>(TOME_ARG)?;
    let grims = args.get_argument::<u64>(GRIM_ARG)?;
    let bonus = args.get_argument::<u64>(BONUS_ARG)?;
    roll_dice(tomes, grims, bonus)?;
    Ok(())
}
