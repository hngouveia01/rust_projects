extern crate rand;
extern crate ncurses;

use rand::Rng;
use ncurses::*;
use std::io;

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
        do_battle(&hero);
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

fn do_battle(hero: &Hero) -> i32 {
    let mut monster_name = String::new();
    let mut monster_hability = String::new();
    let mut monster_energy = String::new();
    let mut again = 1;
    let mut monster_stats: (i32, i32) = (0, 0);

    printw("Monster's name: ");
    refresh();
    echo();

    getstr(&mut monster_name);

    while again == 1 {
        clear();
        printw("Monster's hability: ");
        refresh();
        getstr(&mut monster_hability);

        let monster_hability: i32 = match monster_hability.trim().parse() {
            Ok(monster_hability) => monster_hability,
            Err(_) => continue,
        };

        if monster_hability <= 0 {
            again = 1;
        } else {
            again = 0;
        }
        monster_stats.0 = monster_hability;
    }

    again = 1;

    while again == 1 {
        clear();
        printw("Monster's energy: ");
        refresh();
        getstr(&mut monster_energy);

        let monster_energy: i32 = match monster_energy.trim().parse() {
            Ok(monster_energy) => monster_energy,
            Err(_) => continue,
        };

        if monster_energy <= 0 {
            again = 1;
        } else {
            again = 0;
        }
        monster_stats.1 = monster_energy;
    }

    let mut monster = Monster::new(monster_name, monster_stats.0, monster_stats.1);
    1
}
