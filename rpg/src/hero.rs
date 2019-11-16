use crate::weapon::*;
use crate::action::*;

pub trait HeroMethods {
    fn get_life(&self) -> i16;
    fn set_life(&mut self);
    fn attack(&self, hero: &mut Hero);
    fn info(&self);
    fn new(life: i16, hero_type: &HeroType, pos: Position) -> Hero;
    fn is_alive(&self) -> bool;
    fn get_pos(&self) -> Position;
    fn set_pos(&mut self, pos: Position);
    fn walk(&mut self);
    fn spell(&self);
}

pub struct Hero {
    pub life: i16,
    pub hero_type: HeroType,
    pub weapon: Weapon,
    pub pos: Position,
}

#[derive(Debug, Copy, Clone)]
pub enum HeroType{
    Knight,
    Archer,
    Mage,
}
#[derive(Debug, Copy, Clone)]
pub struct Position{
    pub x: u8,
    pub y: u8,
}

impl HeroMethods for Hero {

    fn get_life(&self) -> i16 {
        self.life
    }
    fn set_life(&mut self) {
        self.life = 100;
    }
    fn attack(&self, hero: &mut Hero) {
        hero.life -= self.weapon.get_atk() as i16;
    }
    fn info(&self) {
        println!("life: {}, hero:{:?}", self.life, self.hero_type);
    }
    fn new(life: i16, hero_type: &HeroType, pos: Position) -> Hero {
        Hero{
            life: life,
            hero_type: *hero_type,
            weapon: match *hero_type {
                HeroType::Knight => Weapon::Sword(30, 1),
                HeroType::Archer => Weapon::Bow(20, 3),
                HeroType::Mage => Weapon::Stick(15, 3),
            },
            pos: pos,
        }
    }
    fn is_alive(&self) -> bool {
        self.get_life() > 0
    }
    fn get_pos(&self) -> Position{
        self.pos
    }
    fn set_pos(&mut self, pos: Position){
        self.pos = pos;
    }
    fn walk(&mut self){

    }
    fn spell(&self){
        
    }

}

