use legion::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon;

// weapons arent like normal items, we dont want the on the option to use list
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeaponInUse(pub Entity);
