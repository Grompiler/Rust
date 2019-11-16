// on declare via le main tous les modules du projet qu'on
// veut utiliser dans le reste de la crate (=le reste des XXX.rs)
mod hero; // "bring files hero.rs & weapon.rs to the scope"
mod weapon; // pour pouvoir utiliser "use crate::weapon::*;" dans hero.rs
mod action;


// on declare simplement ce qu'on utilise directement dans le main
use hero::*;


fn main() {

 
    let mut first_hero = Hero::new(200, &HeroType::Knight, Position{x:0, y:0});
    first_hero.info();

    let mut second_hero = Hero::new(210, &HeroType::Knight, Position{x:10, y:10});
    second_hero.info();

    
    while first_hero.is_alive() && second_hero.is_alive() {
        first_hero.attack(&mut second_hero);
        if second_hero.is_alive(){second_hero.attack(&mut first_hero);}

        first_hero.info();
        second_hero.info();
    }

        
    println!("end, the winner is ");

}
