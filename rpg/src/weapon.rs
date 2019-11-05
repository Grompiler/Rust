
pub trait WeaponMethods {
    fn new(atq: u8, weapon_type: WeaponType) -> Weapon;
}

pub struct Weapon {
    pub atk: u8,
    pub weapon_type: WeaponType,
}

pub enum WeaponType{
    Sword,
    Bow,
    Stick,
}

impl WeaponMethods for Weapon {
    fn new(atk: u8, weapon_type: WeaponType) -> Weapon {
        Weapon {
            atk: atk,
            weapon_type: weapon_type,
        }
    }
}