pub mod chargenseq;
use crate::attacks::Attack;
use crate::items::*;
use crate::{ElementalEffects, Stats, LEVEL_CAP};

use crate::input;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Proficiency {
	None,
}

// Hero definition. Included to make Clippy shut up.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Hero<'a> {
	pub name: &'a str,

	// Battle Bits
	// TODO: Make it so that you don't have to set this value manually each time
	pub max_hp: u16,
	pub hp: u16,
	pub max_mp: u16,
	pub mp: u16,

	pub stats: Stats,
	pub elements: ElementalEffects<'a>,
	pub equipment: Equipment<'a>,
	//pub charisma: u8, // Uncomment if deemed necessary

	// For the final game, EXP points and experience negotiation should be here.
	pub exp: u16,
	pub movelist: &'a [Attack<'a>], // Functions are down in the impl
}

// Impl definition to allow for functions
impl<'a> Hero<'a> {
	// Adds experience and checks if you can gain a level
	pub fn gain_exp(&mut self, exp: u16) {
		// Because experience is stored as a u16, convert the level value to one
		let level: u16 = self.stats.level.into();

		if self.stats.level < LEVEL_CAP {
			self.exp += exp;

			// A proper level curve should be here
			loop {
				if self.exp >= (10 * level + level) {
					// Lower experience points by the EXP formula
					self.exp -= 10 * level + level;
					self.gain_level()
				} else {
					break;
				}
			}
		}
	}

	// Used to force level ups
	pub fn gain_level(&mut self) {
		if self.stats.level > LEVEL_CAP {
			println!("{}'s level cannot get any higher.", self.name)
		} else {
			self.stats.level += 1;
			println!("Level up! {} is now level {}.", self.name, self.stats.level);
			// Prompt to increase a stat
			loop {
				println!(
					// Manually justified the following lines so they appear evenly matched, allowing for easier judging of stats. Does not account for numbers with more than one digit though.
					"Coose a stat to increase:
                1)     STRENGTH - {} - [{}]
                2)    DEXTERITY - {} - [{}]
                3) CONSTITUTION - {} - [{}]
                4) INTELLIGENCE - {} - [{}]
                5)       SPIRIT - {} - [{}]
                ",
					self.stats.strength,
					"*".repeat(usize::from(self.stats.strength)),
					self.stats.dexterity,
					"*".repeat(usize::from(self.stats.dexterity)),
					self.stats.constitution,
					"*".repeat(usize::from(self.stats.constitution)),
					self.stats.intelligence,
					"*".repeat(usize::from(self.stats.intelligence)),
					self.stats.spirit,
					"*".repeat(usize::from(self.stats.spirit))
				);

				let choice: u8 = input().get();

				match choice {
					1 => {
						self.stats.strength += 1;
						println!(
							"{} feels strong! Strength increased to {}\n",
							self.name, self.stats.strength
						);
						break;
					}

					2 => {
						self.stats.dexterity += 1;
						println!(
							"{} feels agile! Dexterity increased to {}\n",
							self.name, self.stats.dexterity
						);
						break;
					}

					3 => {
						self.stats.constitution += 1;
						println!(
							"{} feels tough! Constitution increased to {}\n",
							self.name, self.stats.constitution
						);
						break;
					}

					4 => {
						self.stats.intelligence += 1;
						println!(
							"{} feels clever! Intelligence increased to {}\n",
							self.name, self.stats.intelligence
						);
						break;
					}

					5 => {
						self.stats.spirit += 1;
						println!(
							"{} feels spiritual! Spirit increased to {}\n",
							self.name, self.stats.spirit
						);
						break;
					}

					_ => {
						println!("Invalid stat number")
					}
				}
			}
		}
	}

	pub fn get_remaining_exp(&self) -> u16 {
		// Store level as u16 for stats
		let level: u16 = self.stats.level.into();
		(10 * level + level) - self.exp
	}
}

// Equipment slot system
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Equipment<'a> {
	pub weapon: Option<Item<'a>>,
	pub offhand: Option<Item<'a>>,
	pub head: Option<Item<'a>>,
	pub armour: Option<Item<'a>>,
	pub legs: Option<Item<'a>>,
	pub accessory: Option<Item<'a>>,
}

impl<'a> Equipment<'a> {
	pub fn equip_item(&mut self, item: Item<'a>) {
		// Decide item slot based on the item passed in
		let slot: EquipSlot = item.equipment_data.unwrap().slot;

		match slot {
			// Equip the item in the necessary slot
			EquipSlot::Head => self.head = Some(item),
			EquipSlot::Armour => self.armour = Some(item),
			EquipSlot::Legs => self.legs = Some(item),
			EquipSlot::Accessory => self.accessory = Some(item),
			EquipSlot::Offhand => self.offhand = Some(item),
			EquipSlot::Weapon => self.weapon = Some(item), // Temporary functionality to account for dual wielding
		}

		println!("Equipped {}", item.name) // TODO: Add an to check names
		                             // Should also remove the item from your inventory if it's in there
	}

	pub fn unequip_item(&mut self, slot: EquipSlot) {
		// Can be called manually.
		match slot {
			// Equip the item in the necessary slot
			EquipSlot::Head => self.head = None,
			EquipSlot::Armour => self.armour = None,
			EquipSlot::Legs => self.legs = None,
			EquipSlot::Accessory => self.accessory = None,
			EquipSlot::Offhand => self.offhand = None,
			EquipSlot::Weapon => self.weapon = None,
			// Should also add the item to your inventory.
		}
	}

	pub fn get_total_ac(&self) -> u8 {
		let mut total_ac: u8 = 0;
		// This code sucks, I need to get a better way to make it
		if self
			.weapon
			.unwrap()
			.equipment_data
			.unwrap()
			.armour_data
			.is_some()
		{
			total_ac += self
				.weapon
				.unwrap()
				.equipment_data
				.unwrap()
				.armour_data
				.unwrap()
				.ac
		}
		if self
			.offhand
			.unwrap()
			.equipment_data
			.unwrap()
			.armour_data
			.is_some()
		{
			total_ac += self
				.offhand
				.unwrap()
				.equipment_data
				.unwrap()
				.armour_data
				.unwrap()
				.ac
		}
		if self
			.head
			.unwrap()
			.equipment_data
			.unwrap()
			.armour_data
			.is_some()
		{
			total_ac += self
				.head
				.unwrap()
				.equipment_data
				.unwrap()
				.armour_data
				.unwrap()
				.ac
		}
		if self
			.armour
			.unwrap()
			.equipment_data
			.unwrap()
			.armour_data
			.is_some()
		{
			total_ac += self
				.armour
				.unwrap()
				.equipment_data
				.unwrap()
				.armour_data
				.unwrap()
				.ac
		}
		if self
			.legs
			.unwrap()
			.equipment_data
			.unwrap()
			.armour_data
			.is_some()
		{
			total_ac += self
				.legs
				.unwrap()
				.equipment_data
				.unwrap()
				.armour_data
				.unwrap()
				.ac
		}
		if self
			.accessory
			.unwrap()
			.equipment_data
			.unwrap()
			.armour_data
			.is_some()
		{
			total_ac += self
				.accessory
				.unwrap()
				.equipment_data
				.unwrap()
				.armour_data
				.unwrap()
				.ac
		}

		// Weapons having an armour class value is uncommon. It's mostly reserved for staves and greatswords.
		total_ac // Return the sum of the AC
	}
}
