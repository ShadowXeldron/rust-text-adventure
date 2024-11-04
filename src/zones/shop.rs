use crate::items::*;
use crate::{input, GlobalData, NPC};

// Shop code
pub struct Shop<'a> {
	pub name: &'a str, // Name of the shop that appears on top of the menu
	pub entry_text: &'a str,
	pub talk_text: &'a str, // Text that is displayed when you select the "talk to shopkeeper" option
	pub shop_type: ShopType, // If a General Store, allow selling. If an Armoury, allow repairing. If a Tavern, allow saving, party management and recruitment. If an Infirmary, allow healing. If a Chapel, allow donation, purification and revival.
	pub inventory: Option<&'a [Item<'a>]>, // What the shop sells. If None, the shop does not sell anything
	pub npcs: Option<&'a [NPC<'a>]>, // Other NPCs in the shop. Mainly reserved for taverns and some of the chapels and adds another menu.
}

impl Shop<'_> {
	// Trade is the basis of capitalism. Capitalism is the basis of greed. Greed is the basis of the human nature.
	pub fn open<'a>(&self, mut global: GlobalData<'a>) -> GlobalData<'a> {
		println!("\n{}\n\"{}\"", self.name, self.entry_text);
		let mut option_count: u8 = 2;

		// Display text as necessary, starting with the top option. This is implementation is more than a bit stupid.
		match self.shop_type {
			ShopType::GeneralStore | ShopType::Armoury => println!("1) Buy Items"), // The game should throw an error if either of these do not have an item for sale
			ShopType::Chapel => println!("1) Ressurect"),
			ShopType::Infirmary => println!("1) Heal"),
			ShopType::Tavern => println!("1) Manage Party"),
		}

		// Second option
		match self.shop_type {
			ShopType::GeneralStore => println!("2) Sell Items"),
			ShopType::Armoury => println!("2) Repair Equipment"),
			ShopType::Chapel => println!("2) Purify"),
			ShopType::Infirmary => println!("2) Cure Status Effects"),
			ShopType::Tavern => println!("2) Recruit New Heroes"),
		}

		if self.shop_type == ShopType::Chapel {
			option_count += 1;
			println!("{}) Donate", option_count);
		}

		if (self.shop_type != ShopType::GeneralStore && self.shop_type != ShopType::Armoury)
			&& self.inventory.is_some()
		{
			option_count += 1;
			println!("{}) Buy Items", option_count);
		}

		option_count += 1;
		println!("{}) Talk to shopkeeper", option_count); // This is a universal feature

		if self.npcs.is_some() {
			option_count += 1;
			println!("{}) Talk to patrons", option_count)
		}

		option_count += 1;
		println!("{}) Exit", option_count);

		// You thought that bit was some stupid code? You've seen nothing yet!

		loop {
			let option = input::<u8>().get();
			match option {
				1 => {
					match self.shop_type {
						ShopType::GeneralStore | ShopType::Armoury => println!("Open the shopping menu"),
						ShopType::Chapel => println!("Ask who wants to get revived"),
						ShopType::Infirmary => println!("Ask who wants to get healed"),
						ShopType::Tavern => println!("Bring up a list of all your recruited teammates.")
					}
				}
				
				2 => {
					match self.shop_type {
						ShopType::GeneralStore => println!("Open the selling menu"),
						ShopType::Armoury => println!("Open the item menu and ask the player to select which damaged item from their inventory they would like to have fixed."),
						ShopType::Chapel => println!("Ask who wants to get their status effects cured"),
						ShopType::Infirmary => println!("Ask who wants to get their status effects cured"),
						ShopType::Tavern => println!("Bring up a list of all your recruited teammates.")
					}
				}
				
				// The last few were relatively tame. Now is when things get stupid. I bet you seventy bajillion gold coins that this is gonna get stupid, and that my maths will be way off in certain areas.
				3 => {
					// Match probably won't work here because we need to be more specific
					if self.shop_type == ShopType::Chapel {println!("Ask for a donation that will tip you towards the chapel's alignment")}
					else if self.shop_type == ShopType::Infirmary && self.inventory.is_some() {println!("Open the shopping menu")}
					else {println!("\"{}\"", self.talk_text)} // Print the shopkeeper dialogue
				}
				
				4 => {
					// This marks the first area when you can actively leave the shop
					if self.shop_type == ShopType::Chapel && self.inventory.is_some() {println!("Open the shopping menu")}
					else if option_count > 4 {
							if self.inventory.is_none() && self.shop_type == ShopType::Chapel {println!("\"{}\"", self.talk_text)}
							else {println!("Pull up a list of NPCs")}
						} // Watch this fail dramatically, in dramatic fashion. Should print shopkeeper dialogue.
					else {break}
				}
				
				5 => {
					if option_count > 5 && (self.shop_type != ShopType::GeneralStore || self.shop_type != ShopType::Armoury) {
						if self.inventory.is_some() && self.shop_type == ShopType::Chapel {println!("\"{}\"", self.talk_text)}
						else {println!("Pull up a list of NPCs")}
					}
					else {break}
				}
				
				6 => {
					// This marks the first area when you can actively leave the shop
					if option_count > 5 && self.shop_type == ShopType::Chapel && self.npcs.is_some() && self.inventory.is_some() {println!("Pull up a list of NPCs")}
					else {break}
				}
				
				7 => break, // This SHOULD be it. Excess values being able to trigger an exit is not an intentional feature, but I've deemed it to be harmless and prob ably faster than doing even more checks so I've left it in.
				// AAAH THIS CODE STINKS 
			
				_ => println!("Invalid option!")
			}
		}

		println!("You left the shop.");
		global // Returns the inserted global. It's kind of a misnomer because it's less global data, moreso hot potato.
	}
}

#[derive(PartialEq)]
pub enum ShopType { // Might need to add another position for magic slots here...
	GeneralStore,
	Armoury,
	Chapel,
	Infirmary,
	Tavern,
}

pub const SAMPLE_CHEAT_SHOP: Shop = Shop {
	name: "The Cheat Shop",
	entry_text: "Welcome to the cheat shop which can be accessed via the cheat codes, you cheater!",
	talk_text: "Structs are the best thing since sliced bread. Enums are much easier to use than they look. Dyon sounds like a cool library that could be super useful for this project but it has far too many dependencies.",
	shop_type: ShopType::GeneralStore,
	inventory: Some(&[ITEM_TONIC, ITEM_POTION, ITEM_ETHER]),
	npcs: None
};

pub const SAMPLE_CHEAT_SHOP2: Shop = Shop {
	name: "The Cheat Shop 2",
	entry_text: "Welcome to the second cheat shop, which we built next to the first cheat shop because money!",
	talk_text: "Unlike the other cheat shop, this one has NPCs.",
	shop_type: ShopType::GeneralStore,
	inventory: Some(&[ITEM_ANTIDOTE, ITEM_BURN_CREAM, ITEM_MEDICINE, ITEM_HOLY_WATER, ITEM_WARM_UP, ITEM_EROSIVE, ITEM_MAGIC_CARROT, ITEM_SPEAKEASY, ITEM_MUSSLES, ITEM_REARMOUR, ITEM_PANCEA]),
	npcs: Some(&[NPC{
		name: "Customer",
		dialogue: "I'm a person who is standing in a shop.",
		fight_table: None
	}])
};

pub const SAMPLE_CHEAT_ARMOURY: Shop = Shop {
	name: "The Cheat Armoury",
	entry_text: "Welcome to the cheat armoury. Our weapon of choice is a GameShark.",
	talk_text: "Structs are the best thing since sliced bread. Enums are much easier to use than they look. Dyon sounds like a cool library that could be super useful for this project but it has far too many dependencies.",
	shop_type: ShopType::Armoury,
	inventory: Some(&[ITEM_WOODEN_SWORD]),
	npcs: None
};

pub const SAMPLE_CHEAT_ARMOURY2: Shop = Shop {
	name: "The Cheat Armoury",
	entry_text: "Welcome to the second cheat armoury. We will protect you from your skill issue.",
	talk_text: "The game should be able to handle even more NPCs than this in one place, but the amount of memory needed adds up FAST. I hate to say it, but perhaps adding *another* library will allow for better use of memory. Loading data from a plain text file should not take long, even on HDDs.",
	shop_type: ShopType::Armoury,
	inventory: Some(&[ITEM_HELM_OF_OPPOSITE_ALIGNMENT]),
	npcs: Some(&[NPC{
		name: "Hat Enthusiast",
		dialogue: "I like wearing hats. I put them on my head all the time, it is very fun.",
		fight_table: None
	},
	NPC{
		name: "Armour Enthusiast",
		dialogue: "The best offense is a good defence.",
		fight_table: None
	},
	NPC{
		name: "Footwear Enthusiast",
		dialogue: "If you don't like shoes, then you should shoo!",
		fight_table: None
	},
	NPC{
		name: "Accessory Enthusiast",
		dialogue: "Accessories usually don't provide AC or MR, but to make up for it they always have a secondary effect!",
		fight_table: None
	},
	NPC{
		name: "Weapon Enthusiast",
		dialogue: "Welcome to Corneria! I like swords.",
		fight_table: None
	},
	NPC{
		name: "Offhand Enthusiast",
		dialogue: "Luckilly, my shield will protect me!",
		fight_table: None
	}])
};

pub const SAMPLE_CHEAT_CHAPEL_LAW: Shop = Shop {
	name: "Lawful Cheat Chapel",
	entry_text: "Welcome to the lawful cheat chapel, where we use Jesus as an excuse to get away with cheating",
	talk_text: "Chapels have alignments and accept donations, sorta like NetHack temples.\nThrow money at us to get temporary bonus AC and shift your alignment score towards -128 for Chaos, 0 for Neutral and 128 for Law.",
	shop_type: ShopType::Chapel,
	inventory: None,
	npcs: None
};

pub const SAMPLE_CHEAT_CHAPEL_LAW2: Shop = Shop {
	name: "Lawful Cheat Megachurch",
	entry_text: "Welcome to the lawful cheat megachurc, where we worship the one true god: CAPITALISM!",
	talk_text: "I still don't know why I decided that Chapels need so many options.",
	shop_type: ShopType::Chapel,
	inventory: Some(&[ITEM_HOLY_WATER, ITEM_BUSTER_SWORD]),
	npcs: Some(&[NPC{
		name: "Church Cat",
		dialogue: "Meow I am a cat",
		fight_table: None
	}, NPC{
		name: "Copyright Notice",
		dialogue: "You wouldn't download a car",
		fight_table: None
	}])
};

pub const SAMPLE_CHEAT_CHAPEL_NEUTRAL: Shop = Shop {
	name: "Neutral Cheat Chapel",
	entry_text: "Welcome to the neutral cheat chapel, because indecision is based",
	talk_text: "What is with MegaTen and alignment bias",
	shop_type: ShopType::Chapel,
	inventory: Some(&[ITEM_HOLY_WATER, ITEM_BUSTER_SWORD]),
	npcs: None
};

pub const SAMPLE_CHEAT_CHAPEL_CHAOS: Shop = Shop {
	name: "Chaotic Cheat Chapel",
	entry_text: "Welcome to the chaotic cheat chapel. Did you know that the devil approves of cheating?",
	talk_text: "WE NEED TO KILL CHAOS IT'S NOT A DREAM IT'S NOT A HOPE IT'S A HUNGER IT'S A THIRST",
	shop_type: ShopType::Chapel,
	inventory: None,
	npcs: Some(&[NPC{
		name: "Jack Frost",
		dialogue: "Hee-ho give me a Life Stone and I will give you a Life Stone in return",
		fight_table: None
	}])
};

pub const SAMPLE_CHEAT_INFIRMARY: Shop = Shop {
	name: "Dr. Cheater's Infirmary",
	entry_text: "Welcome to the cheat infirmary. We heal your health while you hurt other people's feelings.",
	talk_text: "The quick brown fox jumped over the lazy dog",
	shop_type: ShopType::Infirmary,
	inventory: None,
	npcs: None
};

pub const SAMPLE_CHEAT_INFIRMARY2: Shop = Shop {
	name: "US Healthcare System",
	entry_text: "Welcome to the US Healthcare System",
	talk_text: "Insert US healthcare joke here.",
	shop_type: ShopType::Infirmary,
	inventory: Some(&[ITEM_HOLY_WATER, ITEM_BUSTER_SWORD]),
	npcs: Some(&[NPC{
		name: "Big Pharma",
		dialogue: "Exploiting the poor for the gain of the elite. That is the one true way.",
		fight_table: None
	}])
};

pub const SAMPLE_CHEAT_INFIRMARY3: Shop = Shop {
	name: "Trauma Centre: Under the Knife",
	entry_text: "Welcome to that one DS game I watched a YouTube video about once",
	talk_text: "As you can see, I'm running out of area name ideas",
	shop_type: ShopType::Infirmary,
	inventory: Some(&[ITEM_HOLY_WATER, ITEM_BUSTER_SWORD]),
	npcs: None
};

pub const SAMPLE_CHEAT_INFIRMARY4: Shop = Shop {
	name: "UK National Health Service",
	entry_text: "Welcome to the NHS. As you can see, we're chronically underfunded.",
	talk_text: "Screw the Tories they keep trying to kill us off",
	shop_type: ShopType::Infirmary,
	inventory: None,
	npcs: Some(&[NPC{
		name: "NHS Computer",
		dialogue: "Please help I am still running Internet Explorer",
		fight_table: None
	}])
};

pub const SAMPLE_CHEAT_TAVERN: Shop = Shop {
	name: "Cheat Tavern",
	entry_text: "Welcome to the cheat tavern! Need a drink?",
	talk_text: "Implementing multiple character support in this game was a nightmare.",
	shop_type: ShopType::Tavern,
	inventory: None,
	npcs: None
};

pub const SAMPLE_CHEAT_TAVERN2: Shop = Shop {
	name: "Cheat Tavern 2",
	entry_text: "Shwmae! Welcome to Wales where it always rains.",
	talk_text: "As you can see, I've COMPLETELY ran out of ideas.",
	shop_type: ShopType::Tavern,
	inventory: Some(&[ITEM_HOLY_WATER, ITEM_BUSTER_SWORD]),
	npcs: Some(&[NPC{
		name: "Beer",
		dialogue: "I'm overrated",
		fight_table: None
	}])
};

pub const SAMPLE_CHEAT_TAVERN3: Shop = Shop {
	name: "Cheat Tavern 3 & Knuckles",
	entry_text: "OH NO!",
	talk_text: "Unlike Sonic I don't chuckle, I'd rather flex my muscles",
	shop_type: ShopType::Tavern,
	inventory: Some(&[ITEM_HOLY_WATER, ITEM_BUSTER_SWORD]),
	npcs: None
};

pub const SAMPLE_CHEAT_TAVERN4: Shop = Shop {
	name: "Cheat Milk Bar",
	entry_text: "Welcome to the cheat milk bar...",
	talk_text: "Damn Christianity and the ESRB!",
	shop_type: ShopType::Tavern,
	inventory: None,
	npcs: Some(&[NPC{
		name: "Not Drunk Person",
		dialogue: "*Hic* I am not drunk. Because I am not drinking alcohol.",
		fight_table: None
	}])
};
