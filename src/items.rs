// Item code

// Constants

//Item types
//pub const ITEMTYPE_

// Equipment slots
pub const EQUIP_HEAD: u8 = 0; // Uses the headgear slot
pub const EQUIP_ARMOUR: u8 = 1; // Uses the body armour slot
pub const EQUIP_LEGS: u8 = 2; // Uses leg slot
pub const EQUIP_WEAPON: u8 = 3; // Uses the weapon or offhand slot
pub const EQUIP_OFFHAND: u8 = 4; // Uses the offhand slot
pub const EQUIP_ACCESSORY: u8 = 5; // Uses the accessory slot

// Misc equip type, used for anything that isn't a weapon or armour
pub const EQUIP_MISC: u8 = 0;

// Weapon types
pub const WEAPON_SWORD: u8 = 1;
pub const WEAPON_GREATSWORD: u8 = 2;
pub const WEAPON_AXE: u8 = 3;
pub const WEAPON_GREATAXE: u8 = 4;
pub const WEAPON_SPEAR: u8 = 5;
pub const WEAPON_LANCE: u8 = 6;
pub const WEAPON_KNIFE: u8 = 7;
pub const WEAPON_BOW: u8 = 8;
pub const WEAPON_CLUB: u8 = 9;
pub const WEAPON_HAMMER: u8 = 10;
pub const WEAPON_STAFF: u8 = 11;
pub const WEAPON_WAND: u8 = 12;
pub const WEAPON_KNUCKLE: u8 = 13;

// Body armour type
pub const ARMOUR_HEAVY: u8 = 1;
pub const ARMOUR_MEDIUM: u8 = 2;
pub const ARMOUR_LIGHT: u8 = 3;

// Materials, used for passing weaknesses and in the crafting system
pub const MATERIAL_WOOD: u8 = 0;
pub const MATERIAL_COPPER: u8 = 1;
pub const MATERIAL_TIN: u8 = 2;
pub const MATERIAL_BRONZE: u8 = 3;
pub const MATERIAL_STONE: u8 = 4;
pub const MATERIAL_IRON: u8 = 5;
pub const MATERIAL_STEEL: u8 = 6;
pub const MATERIAL_SILVER: u8 = 7;
pub const MATERIAL_GOLD: u8 = 8;
pub const MATERIAL_CRYSTAL: u8 = 9;
pub const MATERIAL_CLOTH: u8 = 10;
pub const MATERIAL_LEATHER: u8 = 11;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Item<'a> {
    // In the interest of being tidy and modular I have introduced too many Somes and dots
    // Flavour
    pub name: &'a str,
    pub description: &'a str,

    // Universal info
    pub is_key_item: bool, // If true, this item cannot be removed from the inventory outside of set events
    pub material: u8,
    pub equipment_data: Option<EquipmentData>, // If None, it can't be equipped.
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct EquipmentData {
    // Requirements
    pub slot: u8,
    pub equip_type: u8, // What thing the equipable is
    pub weight: u8,
    pub is_cursed: bool,

    pub weapon_data: Option<WeaponData>,
    pub armour_data: Option<ArmourData>,
}

// Struct defining information about weapon types
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct WeaponData {
    pub weapon_power: u8,
    pub spell_mod: u8, // Spellcasting Modifier
    pub init_mod: u8,
    pub hit_rate: u8,
    pub element_override: Option<u8>, // Overrides the element used for the flat attack. If None, bases element on the weapon type
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct ArmourData {
    pub ac: u8, // Armour class
    pub mr: u8, // Magic Resistance
                //pub reverse_mr: bool // Potential cheat to reverse
}

pub const ITEM_IRON_SWORD: Item = Item {
    name: "Iron Sword",
    description:
        "A metallic shortsword made out of wrought iron. Not particularly difficult to wield.",

    is_key_item: false,
    material: MATERIAL_IRON,
    equipment_data: Some(EquipmentData {
        slot: EQUIP_WEAPON,
        equip_type: WEAPON_SWORD,
        weight: 5,
        is_cursed: false,
        weapon_data: Some(WeaponData {
            weapon_power: 15,
            spell_mod: 0,
            init_mod: 0,
            hit_rate: 95,
            element_override: None,
        }),
        armour_data: None,
    }),
};
