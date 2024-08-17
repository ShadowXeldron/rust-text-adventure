//mod combat;
use crate::combat::*;
use crate::combat::attacks::*;
use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Mob<'a> {
    // Battle stuff that I need to figure out how to autoassign
    pub max_hp: u16,
    pub hp: u16,
    pub max_mp: u16,
    pub mp: u16,

    pub name: &'a str,
    pub exp_reward: u16,
    pub stats: Stats,
    pub hp_mod: u16, // Additional HP on top of the standard HP calcualtions
    pub elements: ElementalEffects<'a>,
    pub movelist: &'a [Attack<'a>]
}

pub static MOB_PEBBLE: Mob<'_> = Mob {
    name: "Pebble",
    exp_reward: 3,
    max_hp: 1,
    hp: 1,
    max_mp: 1,
    mp: 1,

    stats: Stats {
        level: 1,
        constitution: 5,
        strength: 5,
        dexterity: 5,
        intelligence: 5,
        spirit: 5,
        ac: 0,
        mr: 0,
        wp: 1,
        sp: 0
    },
    hp_mod: 30,

    elements: ElementalEffects {
        weak: Some(&[TYPE_IMPACT, TYPE_ICE, TYPE_GROUND]),
        resist: Some(&[TYPE_SLASH, TYPE_WIND, TYPE_ELECTRIC]),
        immune: None,
        heal: None, // Some(&[TYPE_SLASH])
        reflect: None,
        avoid: None,
    },

    movelist: &[ATTACK_TACKLE, ATTACK_HEAL]

};

pub static MOB_GOBLIN: Mob<'_> = Mob {
    name: "Goblin",
    exp_reward: 5,
    max_hp: 1,
    hp: 1,
    max_mp: 1,
    mp: 1,

    stats: Stats {
        level: 2,
        constitution: 3,
        strength: 6,
        dexterity: 9,
        intelligence: 2,
        spirit: 1,
        ac: 0,
        mr: 0,
        wp: 1,
        sp: 0
    },
    hp_mod: 0,

    elements: ElementalEffects {
        weak: Some(&[TYPE_SLASH, TYPE_ICE]),
        resist: None,
        immune: None,
        heal: None, // Some(&[TYPE_SLASH])
        reflect: None,
        avoid: None,
    },

    movelist: &[ATTACK_STAB]

};

pub static MOB_GRUE: Mob<'_> = Mob {
    name: "Grue",
    max_hp: 0xff,
    hp: 0xff,
    max_mp: 0xff,
    mp: 0xff,
    exp_reward: 120, // Anticlimactic, isn't it? Well, it's because you aren't supposed to fight the Grue. It's only killable for lore reasons - it's harmed by light (literal light; holy magic is classified as such because it also produces light) and under most circumstances it'll kill you in a single hit.

    stats: Stats {
        level: 99,
        constitution: 255,
        strength: 255,
        dexterity: 4,
        intelligence: 0xff,
        spirit: 0xff,
        ac: 0xff,
        mr: 0xff,
        wp: 0xff,
        sp: 0xff
    },
    hp_mod: 0,

    elements: ElementalEffects {
        weak: Some(&[TYPE_LIGHT]),
        resist: None,
        immune: Some(&[TYPE_NEUTRAL, TYPE_SLASH, TYPE_PIERCE, TYPE_IMPACT, TYPE_FIRE, TYPE_ELECTRIC, TYPE_ICE, TYPE_GROUND, TYPE_WIND]),
        heal: Some(&[TYPE_DARK]), // Some(&[TYPE_SLASH])
        reflect: None,
        avoid: None,
    },

    movelist: &[ATTACK_DEVOUR]

};

// Monster classes

//MOBCLASS_NAMES = &["Monster", "Stoneforme"]

//MOBCLASS_MONSTER = 0 // Generic fallback class





































