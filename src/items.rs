// Item code
use crate::{Alignment, Element};

// Constants

//Item types
//pub const ITEMTYPE_

// Equipment slots
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum EquipSlot {
	Head,      // Uses the headgear slot
	Armour,    // Uses the body armour slot
	Legs,      // Uses leg slot
	Weapon,    // Uses the weapon or offhand slot
	Offhand,   // Uses the offhand slot
	Accessory, // Uses the accessory slot
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum EquipType {
	// Weapon Types
	Misc,
	Sword,
	Greatsword,
	Axe,
	Greataxe,
	Spear,
	Lance,
	Knife,
	Bow,
	Club,
	Hammer,
	Staff,
	Wand,
	Knuckle,

	// Body armour type
	HeavyArmour,
	MediumArmour,
	LightArmour,
}

// Materials, used for passing weaknesses and in the crafting system
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Material {
	Wood,   // Passes a fire weakness
	Copper, // Passes an electricity weakness
	Tin,    // Passes an electricity weakness
	Bronze, // Passes an electricity weakness
	Stone,
	Iron,   // Passes an electricity weakness
	Steel,  // Passes an electricity weakness
	Silver, // Passes an electricity weakness
	Gold,   // Passes an electricity weakness
	Crystal,
	Cloth,   // Passes a fire weakness and an ice resistance
	Leather, // Passes a fire weakness and an ice resistance
	Other,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Item<'a> {
	// In the interest of being tidy and modular I have introduced too many Somes and dots
	// Flavour
	pub name: &'a str,
	pub description: &'a str,

	// Universal info
	pub is_key_item: bool, // If true, this item cannot be removed from the inventory outside of set events
	pub material: Material,
	pub equipment_data: Option<EquipmentData>, // If None, it can't be equipped.
	pub value: u32 // Coin value of the item
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct EquipmentData {
	// Requirements
	pub slot: EquipSlot,       // What slot the equipable goes in
	pub equip_type: EquipType, // What thing the equipable is
	pub weight: u8,
	pub is_cursed: bool,
	pub required_alignment: Option<Alignment>,

	pub weapon_data: Option<WeaponData>,
	pub armour_data: Option<ArmourData>,
}

impl EquipmentData {
	pub fn get_weapon_element(&self) -> Element {
		if self.weapon_data.unwrap().element_override.is_some() {
			return self.weapon_data.unwrap().element_override.unwrap();
		} else {
			match self.equip_type {
				EquipType::Sword | EquipType::Greatsword | EquipType::Axe | EquipType::Greataxe => {
					Element::Slash
				}
				EquipType::Spear | EquipType::Lance | EquipType::Knife | EquipType::Bow => {
					Element::Pierce
				} // Knives should have unique handling.
				EquipType::Club | EquipType::Hammer | EquipType::Staff => Element::Impact,
				_ => Element::Neutral,
			}
		}
	}
}

// Struct defining information about weapon types
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct WeaponData {
	pub weapon_power: u8,
	pub spell_mod: u8, // Spellcasting Modifier
	pub init_mod: u8,
	pub hit_rate: u8,
	pub element_override: Option<Element>, // Overrides the element used for the flat attack. If None, bases element on the weapon type
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct ArmourData {
	pub ac: u8, // Armour class
	pub mr: u8, // Magic Resistance
}

// ITEMS

// HP Recovery
pub const ITEM_TONIC: Item = Item {
	name: "Tonic",
	description:
		"Weak curative formula. Drinking this will cause your wounds to close.\nRestores 20 HP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 20,
};

pub const ITEM_POTION: Item = Item {
	name: "Potion",
	description: "Moderate curative formula. Drinking this can fix broken bones.\nRestores 50 HP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 50,
};

pub const ITEM_SUPER_POTION: Item = Item {
	name: "Super Potion",
	description:
		"Strong curative formula. Drinking this will regrow amputated apendages.\nRestores 100 HP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 100,
};

pub const ITEM_HYPER_POTION: Item = Item {
	name: "Hyper Potion",
	description:
		"Intense curative formula. Drinking this will bring somebody back from a vegetative state.\nRestores 200 HP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 200,
};

pub const ITEM_OMEGA_POTION: Item = Item {
	name: "Omega Potion",
	description:
		"Extreme curative formula. Drinking this will restore bald hair.\nFully restores HP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 3000,
};

// MP Recovery
pub const ITEM_LESSER_ETHER: Item = Item {
	name: "Lesser Ether",
	description:
		"Liquid mana leftover from an alchemical process that makes potions. There's a bit of magic left in it.\nRestores 30 MP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 35,
};

pub const ITEM_ETHER: Item = Item {
	name: "Lesser Ether",
	description:
		"Magical fluid produced during alchemical transmutation of metals. It's still actively magical.\nRestores 80 MP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 85,
};

pub const ITEM_HIGH_ETHER: Item = Item {
	name: "High Ether",
	description: "Byproduct of intense alchemy. It's still actively magical.\nRestores 100 MP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 105,
};

pub const ITEM_GREAT_ETHER: Item = Item {
	name: "Great Ether",
	description:
		"The purest form of magical energy converted into a liquid form.\nFully restores MP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 3500,
};

pub const ITEM_ELIXR: Item = Item {
	name: "Elixr",
	description:
		"The ultimate alchemical formula which requires a Philosophers' Stone to produce.\nFully restores HP and MP.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 7500,
};

// Status curatives
// Non-Volatile
pub const ITEM_ANTIDOTE: Item = Item {
	name: "Antidote",
	description:
		"General purpose reversal agent which can expunge the majority of common toxins.\nCures poison.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 25,
};

pub const ITEM_BURN_CREAM: Item = Item {
	name: "Burn Cream",
	description: "Slave which can applied to burns reverse their effects.\nCures burn.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 20,
};

pub const ITEM_MEDICINE: Item = Item {
	name: "Medicine",
	description:
		"Medicinal drug produced through alchemy which can cure most common diseases.\nCures illness.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 35,
};

pub const ITEM_HOLY_WATER: Item = Item {
	name: "Holy Water",
	description:
		"Liquid blessed by those in service to the God of Order. It actively weakens malevolent witchcraft.\nBreaks any curses on the user and unequips all cursed equipment.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 50, // Church mandated tax. They're a bunch of capitalisitic jerks in this game.
};

pub const ITEM_WARM_UP: Item = Item {
	name: "Warm-Up",
	description:
		"Alchemical fluid that causes anyone who drinks it to quickly warm up.\nCures chill and freeze.", // Freeze is a volatile status, but both it and chill are related to the ice element.
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 30,
};

pub const ITEM_EROSIVE: Item = Item {
	name: "Erosive",
	description:
		"Blessed liquid that, when poured on petrified flesh, will return it back to normal.\nCures petrification.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 80,
};

pub const ITEM_MAGIC_CARROT: Item = Item {
	name: "Magic Carrot",
	description: "Enchanted vegetable that rabbits crave for.\nCures lagomorph.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 70,
};

pub const ITEM_SPEAKEASY: Item = Item {
	name: "Speakeasy",
	description: "Beverage that makes people talk.\nCures silence.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 40,
};

pub const ITEM_MUSSLES: Item = Item {
	name: "Mussels",
	description: "Edible mollusc which fixes up damaged tissue when consumed.\nCures weak.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 25,
};

pub const ITEM_REARMOUR: Item = Item {
	name: "ReArmour",
	description:
		"Magical device which can fix up damaged armour on an alchemical level.\nCures broken.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 25,
};

pub const ITEM_PANCEA: Item = Item {
	name: "Pancea",
	description: "Miracle formula which can cure any ailment.\nCures all status effects.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 150,
};

// Volatile
pub const ITEM_SLAP_SYRUP: Item = Item {
	name: "Slap Syrup",
	description:
		"Strong-tasting syrup that's bound to knock some sense into anyone who drinks it.\nCures charm, confusion and rage.", // Should ideally have a Patra equivalent with the same effect.
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 35,
};

pub const ITEM_CLOCKCORN: Item = Item {
	name: "Clockcorn",
	description:
		"Maize that makes a ticking sound if you listen to it closely enough.\nCures halt.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 50,
};

pub const ITEM_BANDAGE: Item = Item {
	name: "Bandage",
	description: "Bandage to be applied to a bleeding wound.\nCures bleeding.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 15,
};

pub const ITEM_DISTUN: Item = Item {
	name: "Distun", // Placeholder name
	description: "Formula that can knock a stunned person back to normal.\nCures stun.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 40,
};

// Revival
pub const ITEM_PHOENIX_FEATHER: Item = Item {
	name: "Phoenix Feather",
	description:
		"The shed feathers of Phoenix, the legendary bird. They are imbued with the power of life.\nBrings a dead party member back to life on 1 HP",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 150,
};

// Field

pub const ITEM_FIREWOOD: Item = Item {
	name: "Firewood",
	description:
		"Lets you set up camp in any overworld area to safely perform long rests. You can also prepare food at a campfire - just remember to finish your meal before the giant catches wind of it...\nCannot be used in buildings or dungeons.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 35,
};

pub const ITEM_SLOP: Item = Item {
	name: "Slop",
	description:
		"Barely edible green stuff that the Giant seems to have conciously decided not to eat. Can't really blame him though - even starving peasants will turn their nose up at this.\nCan be consumed while camping. Fully restores your saturation, but eating it may make you sick.", // Sickness is a decided by a DC5 constitution saving throw
	is_key_item: false,
	material: Material::Other,
	equipment_data: None,
	value: 5,
};

// Weaponry

// Swords
// No proficiency required
pub const ITEM_WOODEN_SWORD: Item = Item {
	name: "Wooden Sword",
	description:
		"Wooden training sword. Easy to weild, but meant more for practice than true combat.",

	is_key_item: false,
	material: Material::Wood,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 20,
};

pub const ITEM_COPPER_SHORTSWORD: Item = Item {
	name: "Copper Shortsword",
	description:
		"Sharpened copper blade that is usually held by beginners. Its short but pointy blade makes it more useful for piercing than it does slashing.",

	is_key_item: false,
	material: Material::Copper,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 1,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 3,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: Some(Element::Pierce),
		}),
		armour_data: None,
	}),
	value: 35,
};

// Basic proficency required
pub const ITEM_BRONZE_GLADIUS: Item = Item {
	name: "Bronze Gladius",
	description: "Short, double edged bronze sword",

	is_key_item: false,
	material: Material::Bronze,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_IRON_SWORD: Item = Item {
	name: "Iron Sword",
	description: "Arming sword forged from wrought iron. Not particularly difficult to wield.",

	is_key_item: false,
	material: Material::Iron,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 80,
};

pub const ITEM_GOLDEN_SWORD: Item = Item {
	name: "Golden Sword",
	description: "Sword fashioned from solid gold. It's flashy, but very heavy.",

	is_key_item: false,
	material: Material::Gold,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 15,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 500,
};

// Intermediate proficency required
pub const ITEM_LONGSWORD: Item = Item {
	name: "Longsword",
	description:
		"Steel cruciform longsword which can be used with either one or two hands. It's somewhat heavy and will take some training to weild correctly.",

	is_key_item: false,
	material: Material::Iron,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 115,
};

pub const ITEM_TALWAR: Item = Item { // This is a reference to the earlier Ys games which feature a Talwar (https://en.wikipedia.org/wiki/Talwar) as a mid-tier weapon.
	name: "Talwar",
	description:
		"Curved steel sabre with a single edge that excels at cutting and thrusting. You'll need to train before you can use it.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 140,
};

pub const ITEM_SHADOWBLADE: Item = Item {
	name: "Shadowblade",
	description:
		"Sword of death fashioned from an otherworldly dark gemstone. Malevolent magicks work their way through the sword and can be seen swirling around within it.",

	is_key_item: false,
	material: Material::Crystal,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 10,
		is_cursed: true,
		required_alignment: Some(Alignment::Chaotic),
		weapon_data: Some(WeaponData {
			weapon_power: 35,
			spell_mod: 10,
			init_mod: 0,
			hit_rate: 95,
			element_override: Some(Element::Dark),
		}),
		armour_data: None,
	}),
	value: 140,
};

pub const ITEM_DIAMOND_SWORD: Item = Item { // Painfully obvious Minecraft reference
	name: "Diamond Sword",
	description:
		"Shining sword made out of subterranean crystals to efficienty combine power and durability. It's a bit heavy.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 760,
};

// Advanced proficiency required
pub const ITEM_YANMAODAO: Item = Item { // Used a Chinese sword instead of a Katana because I think they're overdone. Don't worry fans of Japanese weapons, you get a spear and an anti-cavalry sword later on.
	name: "Yanmaodao",
	description:
		"Lightly curved sabre requiring a great deal of expertise to weild correctly. It combines the best qualities of two other kinds of swords for maximum effectiveness.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: Some(Element::Fire),
		}),
		armour_data: None,
	}),
	value: 180,
};

pub const ITEM_ENCHANTMENT_SWORD: Item = Item {
	// Slightly less obvious Minecraft reference
	name: "Enchantment Sword",
	description: "Mages' sword entwined with dozens of enchantments to increase its effectvieness.",

	is_key_item: false,
	material: Material::Silver,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 230,
};

pub const ITEM_LASERBLADE: Item = Item { // Painfully obvious Star Wars reference
	name: "Laserblade",
	description:
		"Curious object that was dropped by an extraterrestrial. It's held like a sword and emits a beam of energy when used. The unorthodox design of this weapon means that it can only be used by experienced swordfighters.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 265,
};

pub const ITEM_DRAGON_SWORD: Item = Item {
	name: "Dragon Sword",
	description: "Weapon crafted from crimson ore. It crackles with flames when unsheathed.",

	is_key_item: false,
	material: Material::Crystal,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: Some(Element::Fire),
		}),
		armour_data: None,
	}),
	value: 50,
};

// Artefact Weapons - Master proficency required. All Artefacts are alignment locked.
// Law-aligned
pub const ITEM_CALEDFWLCH: Item = Item {
	name: "Caledfwlch",
	description: "Blessed blade whose weilder is destined to be crowned king",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Lawful),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 101,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 500,
};

pub const ITEM_CLAIOMH_SOLAIS: Item = Item {
	name: "Claíomh Solais",
	description: "Shining blade that glows with an undying light",

	is_key_item: false,
	material: Material::Gold,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Lawful),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 101,
			element_override: Some(Element::Light),
		}),
		armour_data: None,
	}),
	value: 500,
};

//Neutral-aligned
pub const ITEM_GRAM: Item = Item {
	name: "Gram",
	description: "Legendary dragon killing sword",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Neutral),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 101,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 500,
};

pub const ITEM_DYRNWYN: Item = Item {
	name: "Dyrnwyn",
	description: "Sword that catches fire when held.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Neutral),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 101,
			element_override: Some(Element::Fire),
		}),
		armour_data: None,
	}),
	value: 500,
};

// Chaos-aligned
pub const ITEM_DAINSLEIF: Item = Item {
	name: "Dáinsleif",
	description: "Bloodthirstly blade that refuses to be sheathed",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Chaotic),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 101,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 500,
};

pub const ITEM_TYRFING: Item = Item {
	name: "Tyrfing",
	description: "Cursed blade that commits acts of evil.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Sword,
		weight: 5,
		is_cursed: true,
		required_alignment: Some(Alignment::Chaotic),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 101,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 500,
};

// Greatswords

// No proficency required
pub const ITEM_WOODEN_GREATSWORD: Item = Item {
	name: "Wooden Greatsword",
	description: "Two handed sword that was carved from wood. It's used for training.",

	is_key_item: false,
	material: Material::Wood,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_STONE_SWORD: Item = Item {
	name: "Stone Sword",
	description:
		"Heavy sword made of stone. Usable by beginners, the material it uses means it crushes rather than cuts.",

	is_key_item: false,
	material: Material::Stone,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: Some(Element::Impact),
		}),
		armour_data: None,
	}),
	value: 50,
};

// Basic proficiency required
pub const ITEM_HEAVY_BRONZE_SWORD: Item = Item {
	name: "Heavy Bronze Sword",
	description:
		"Towering sword made from bronze. It requires a bit of training to weild correctly.",

	is_key_item: false,
	material: Material::Wood,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_CLAYMORE: Item = Item {
	name: "Claymore",
	description: "Incredibly heavy iron greatsword. You'll need to practice before you can use it.",

	is_key_item: false,
	material: Material::Iron,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_WEREBREAKER: Item = Item {
	name: "Werebreaker",
	description: "Silver greatsword used in battle against lycanthropes.",

	is_key_item: false,
	material: Material::Silver,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

// Intermediate proficency required
pub const ITEM_ZWEIHANDER: Item = Item {
	name: "Zweihänder",
	description:
		"Towering sword made from bronze. It requires a bit of training to weild correctly.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_NODACHI: Item = Item {
	name: "Nodachi",
	description:
		"Huge curved sword for battling cavalry. You'll need a fair bit of proficiency in order to use it.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_DEVIL_SWORD: Item = Item {
	name: "Devil Sword",
	description: "Blood-red Nodachi overflowing with demonic power.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: true,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_CHANGDAO: Item = Item {
	name: "Changdao",
	description:
		"Humongous two-handed sword with a single edge. Lighter than it looks, but you're going to need a fair bit of training to use it.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: true,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

// Advanced proficency required
pub const ITEM_BEARING_SWORD: Item = Item {
	name: "Bearing Sword",
	description: "Ceremonial weapon that is ridiculously large.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: true,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_PALADINS_GREATSWORD: Item = Item {
	name: "Paladin's Greatsword",
	description:
		"Gigantic greatsword that protects the weilder so that they in turn can protect the weak.",

	is_key_item: false,
	material: Material::Crystal,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: true,
		required_alignment: Some(Alignment::Lawful),
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: Some(ArmourData { ac: 20, mr: 50 }),
	}),
	value: 50,
};

pub const ITEM_BUSTER_SWORD: Item = Item {
	name: "Buster Sword",
	description:
		"Massive heap of raw iron with a singular cutting edge. It's gigantically heavy and demands a lot of training to weild",

	is_key_item: false,
	material: Material::Iron,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: true,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_THANATOS: Item = Item {
	// It's a Thanoscopter Sword
	name: "Thanatos",
	description: "Gigantic sword with two blades on both ends.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 3,
		is_cursed: false,
		required_alignment: None,
		weapon_data: Some(WeaponData {
			weapon_power: 2,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 95,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

// Artifact weapons. These have an alignment score requirement in order to weild
// I looked at legendary weapons on Wikipedia for these, ehehehe

pub const ITEM_DURENDAL: Item = Item {
	// Terraria reference
	name: "Durendal",
	description: "Incredibly sharp greatsword used by a legendary paladin.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Lawful),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 75,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_LAHAT_CHEREB: Item = Item {
	name: "Lahat Chereb",
	description: "Flaming sword that blocks the entrance to paradise.",

	is_key_item: false,
	material: Material::Crystal,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Lawful),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 75,
			element_override: Some(Element::Fire),
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_TIZONA: Item = Item {
	name: "Tizona",
	description: "Sword that strikes fear into the hearts of the unworthy.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Neutral),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 75,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_CALDABOLG: Item = Item {
	name: "Caldabolg",
	description: "Greatsword that can cut the tops from hills",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Neutral),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 70,
			element_override: Some(Element::Earth),
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_SKOFNUNG: Item = Item {
	name: "Skofnung",
	description: "Greatsword with twelve berserker spirits bound to it", // Yes I know that Wikipedia marked this as citation needed but it sounded too cool not to include

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 5,
		is_cursed: false,
		required_alignment: Some(Alignment::Chaotic),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 75,
			element_override: Some(Element::Dark),
		}),
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_MURAMASA: Item = Item {
	name: "Muramasa",
	description: "Raging blade with the heart of a demon.",

	is_key_item: false,
	material: Material::Steel,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Weapon,
		equip_type: EquipType::Greatsword,
		weight: 5,
		is_cursed: true,
		required_alignment: Some(Alignment::Chaotic),
		weapon_data: Some(WeaponData {
			weapon_power: 15,
			spell_mod: 0,
			init_mod: 0,
			hit_rate: 75,
			element_override: None,
		}),
		armour_data: None,
	}),
	value: 50,
};

// Armour
pub const ITEM_HELM_OF_OPPOSITE_ALIGNMENT: Item = Item { // This should invert all alignment calculations while the main character is wearing it
	name: "Helm of Opposite Alignment",
	description:
		"Cursed helmet that oscilates the minds of anybody who puts it on.\nIf this helmet is equipped to any party members, your alignment score will be inverted during the majority of alignment checks.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Armour,
		equip_type: EquipType::Misc,
		weight: 5,
		is_cursed: true,
		required_alignment: Some(Alignment::Chaotic),
		weapon_data: None,
		armour_data: None,
	}),
	value: 50,
};

pub const ITEM_HELM_OF_UNALIGNMENT: Item = Item { // This should treat the alignmenmt score as if it is 0 if any character is wearing it.
	name: "Helm of Unalignment",
	description:
		"Cursed helmet that eradicates the wearer's morality.\nIf this helmet is equipped to any party members, most alignment checks will act as if your alignment score is 0.",
	is_key_item: false,
	material: Material::Other,
	equipment_data: Some(EquipmentData {
		slot: EquipSlot::Armour,
		equip_type: EquipType::Misc,
		weight: 5,
		is_cursed: true,
		required_alignment: Some(Alignment::Chaotic),
		weapon_data: None,
		armour_data: None,
	}),
	value: 50,
};
