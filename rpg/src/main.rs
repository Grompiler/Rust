mod hero;
mod weapon;

use hero::*;
use weapon::*;


fn main() {

    let first_weapon = Weapon::new(50, WeaponType::Sword);

    let mut first_hero = Hero::new(200, HeroType::Knight, first_weapon);
    first_hero.info();

    let second_weapon = Weapon::new(50, WeaponType::Sword);
    let mut second_hero = Hero::new(210, HeroType::Knight, second_weapon);
    second_hero.info();

    
    while first_hero.is_alive() && second_hero.is_alive() {
        first_hero.attack(&mut second_hero);
        second_hero.attack(&mut first_hero);

        first_hero.info();
        second_hero.info();
    }

        
    println!("end, the winner is ");

}
