pub trait WeaponMethods{
    fn get_atk(&self) -> u8;
    fn get_range(&self) -> u8;

}

pub enum Weapon{
    Sword(u8, u8),
    Bow(u8, u8),
    Stick(u8, u8),
}

impl WeaponMethods for Weapon {
    fn get_atk(&self) -> u8 {
        match self {
            Weapon::Sword(atk, _) => *atk,
            Weapon::Bow(atk, _) => *atk,
            Weapon::Stick(atk, _) => *atk,
        }
    }
    fn get_range(&self) -> u8 {
        match self {
            Weapon::Sword(_, range) => *range,
            Weapon::Bow(_, range) => *range,
            Weapon::Stick(_, range) => *range,
        }
    }
}
