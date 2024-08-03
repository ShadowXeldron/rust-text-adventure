// Constants

const EQUIP_HEAD: usize = 0;
const EQUIP_ARMOUR: usize = 1;
const EQUIP_LEGS: usize = 2;
const EQUIP_WEAPON: usize = 3;
const EQUIP_OFFHAND: usize = 4;
const EQUIP_ACCESSORY: usize = 5;

const WEAPON_SWORD: usize = 0;
const WEAPON_GREATSWORD: usize = 1;
const WEAPON_AXE: usize = 2;
const WEAPON_GREATAXE: usize = 3;
const WEAPON_SPEAR: usize = 4;
const WEAPON_LANCE: usize = 5;
const WEAPON_KNIFE: usize = 6;
const WEAPON_BOW: usize = 7;
const WEAPON_CLUB: usize = 8;
const WEAPON_HAMMER: usize = 9;
const WEAPON_STAFF: usize = 10;
const WEAPON_WAND: usize = 11;
const WEAPON_KNUCKLE: usize = 12;

const ARMOUR_HEAVY: usize = 0;
const ARMOUR_MEDIUM: usize = 1;
const ARMOUR_LIGHT: usize = 2;

struct Equipment {
    // Flavour
    name: &str,
    description: &str,
    
    // Requirements
    slot: usize,
    equip_type: usize, // What thing the equipable is
    weight: usize,
    is_cursed: bool,

    // Defensive
    armour_class: usize,
    magic_resistance: isize,

    // Offensive
    weapon_power: isize,
    spell_mod: isize, // Spellcasting Modifier
    init_mod: isize,
    hit_rate: usize

    // Insert others here
};

const iron_sword = Equipment {
    name: "Iron Sword",
    description: "A metallic shortsword made out of wrought iron. Not particularly difficult to wield",
    slot: EQUIP_WEAPON,
    equip_type: WEAPON_SWORD,
    weight: 5,
    is_cursed: false,
    weapon_power: 10
    hit_rate: 95,
} ;