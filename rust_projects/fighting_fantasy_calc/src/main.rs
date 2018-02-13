extern crate rand;
use rand::Rng;

#[allow(dead_code)]

struct HeroStat {
    hability: i32,
    energy: i32,
    luck: i32,
}

struct MonsterStat {
    hability: i32,
    energy: i32,
}

struct Hero {
    initial_stat: HeroStat,
    actual_stat: HeroStat,
}

impl Hero {
    fn new() -> Hero {
        let hability: i32 = run_one_dice_d6() + 6;
        let energy: i32 = run_two_dices_d6() + 12;
        let luck: i32 = run_one_dice_d6() + 6;

        Hero {
            initial_stat: HeroStat {
                              hability: hability,
                              energy: energy,
                              luck: luck
                          },

            actual_stat: HeroStat {
                             hability: hability,
                             energy: energy,
                             luck: luck
            }

        }
    }
}

fn main() {
    let mut hero = Hero::new();
}

fn run_one_dice_d6() -> i32 {
    rand::thread_rng().gen_range(1, 6)
}

fn run_two_dices_d6() -> i32 {
    let d1: i32 = run_one_dice_d6();
    let d2: i32 = run_one_dice_d6();

    d1 + d2
}