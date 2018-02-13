extern crate rand;
extern crate ncurses;

use rand::Rng;
use ncurses::*;

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

struct Monster {
    name: String,
    initial_stat: MonsterStat,
    actual_stat: MonsterStat,
}

impl Monster {
    fn new(name: String, hability: i32, energy: i32) -> Monster {
        Monster {
            name: name,
            initial_stat: MonsterStat {
                              hability: hability,
                              energy: energy
                          },
            actual_stat: MonsterStat {
                             hability: hability,
                             energy: energy
                         }
        }
    }
}

fn main() {
    let mut hero = Hero::new();
    let mut again = 1;

    initscr();

    printw("Your hero initial status:\n");
    refresh();

    printw("-----------------------------------------------\n");
    print_hero_status(&hero);
    printw("-----------------------------------------------\n\n");

    while again == 1 {
        printw("Press any key to start a battle!\n");
        refresh();
        getch();
        clear();
    }
}

fn run_one_dice_d6() -> i32 {
    rand::thread_rng().gen_range(1, 6)
}

fn run_two_dices_d6() -> i32 {
    let d1: i32 = run_one_dice_d6();
    let d2: i32 = run_one_dice_d6();

    d1 + d2
}

fn print_hero_status(hero: &Hero) {
    printw(format!("Initial Hability: {}\n", hero.initial_stat.hability).as_ref());
    printw(format!("Initial Energy: {}\n", hero.initial_stat.energy).as_ref());
    printw(format!("Initial Luck: {}\n", hero.initial_stat.luck).as_ref());
    refresh();
}










