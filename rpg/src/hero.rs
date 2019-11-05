use crate::weapon::*;

pub trait HeroMethods {
    fn get_life(&self) -> u8;
    fn set_life(&mut self);
    fn attack(&self, hero: &mut Hero);
    fn info(&self);
    fn new(life: u8, hero_type: HeroType, weapon: Weapon) -> Hero;
    fn is_alive(&self) -> bool;
}

pub struct Hero {
    pub life: u8,
    pub hero_type: HeroType,
    pub weapon: Weapon,
}

#[derive(Debug)]
pub enum HeroType{
    Knight,
    Archer,
    Mage,
}

impl HeroMethods for Hero {

    fn get_life(&self) -> u8 {
        self.life
    }
    fn set_life(&mut self) {
        self.life = 100;
    }
    fn attack(&self, hero: &mut Hero) {
        hero.life -= self.weapon.atk;
    }
    fn info(&self) {
        println!("life: {}, hero:{:?}", self.life, self.hero_type);
    }
    fn new(life: u8, hero_type: HeroType, weapon: Weapon) -> Hero {
        Hero{
            life: life,
            hero_type: hero_type,
            weapon: weapon,
        }
    }
    fn is_alive(&self) -> bool {
        self.get_life() > 0
    }

}

