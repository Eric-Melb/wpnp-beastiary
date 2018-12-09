use std::fs::OpenOptions;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate rand;

mod helpers;

mod beastio;
use beastio::*;

mod beastiary;
use beastiary::*;

mod monster;
use monster::*;

mod encounter;
use encounter::*;

// TODO: replace many unwraps() with actual error handling
// TODO: implement len for structs

// bunch of constants for the witcher
const STATISTIC_NAMES: [&str; 9] = ["int", "ref", "dex", "body", "spd", "emp", "cra", "will", "luck"];
const DERIVED_STATISTIC_NAMES: [&str; 8] = ["stun", "run", "leap", "sta", "enc", "rec", "hp", "vigor"];
const MONSTER_STATISTIC_NAMES: [&str; 3] = ["height", "weight", "intelligence"];

const SKILL_NAMES: [&str; 19] = ["spell_casting", "archery", "staff_or_spear", "swordsmanship",
"crossbow", "small_blades", "melee", "brawling", "hex_weaving", "ritual_crafting",
"dodge_or_escape", "athletics", "awareness", "stealth", "wilderness_survival",
"resist_magic", "resist_coercion", "endurance", "courage"];

const BEASTIARY_JSON: &str = "beastiary.json";
const AFTER_READ_JSON: &str = "beastiary.json.backup";

fn main()
{

        // TODO - should probably only remove the old beastiary when we write a new one, not before (just copy instead of rename?)
        // TODO - catch error if file doesn't exist and create a blank beastiary instead
        let mut the_beastiary = Beastiary::new(BEASTIARY_JSON, AFTER_READ_JSON);

        // TODO: debug-delete
        the_beastiary.check_beasts();

        // TODO - how expensive is pushing here compared to this being static?
        let mut main_menu: Vec<(u8, &str)> = Vec::new();
        main_menu.push((1, "Generate Encounter"));
        main_menu.push((2, "Roll Initiative"));
        main_menu.push((3, "Make Attacks"));
        main_menu.push((4, "Record Damage"));
        main_menu.push((5, "Add to Beastiary"));
        main_menu.push((6, "Remove from Beastiary"));
        main_menu.push((7, "Quit"));

        // menu loop
        loop
        {
                let choice = print_menu(&main_menu);

                match choice
                {
                        1 => (generate_encounter(&mut the_beastiary)),
                        2 => (),
                        3 => (),
                        4 => (),
                        5 => add_to_beastiary(&mut the_beastiary),
                        6 => (),
                        7 => break,
                        _ => ()
                }

        }

        // writes the beastiary when we quit so that even if no beasts were entered we get the file back
        // there's deffo a better way to do this - TODO: return to this once other functions are in
        let outfile = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("beastiary.json")
        .unwrap();

        serde_json::to_writer(outfile, &the_beastiary.beasts).unwrap();

}



fn add_to_beastiary(the_beastiary: &mut Beastiary)
{
        loop
        {
                the_beastiary.add_beast(enter_monster(STATISTIC_NAMES, DERIVED_STATISTIC_NAMES, MONSTER_STATISTIC_NAMES, SKILL_NAMES));

                // TODO: debug-delete
                the_beastiary.check_beasts();

                if !keep_going()
                {
                        break;
                }
        }

        let outfile = OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open("beastiary.json")
                .unwrap();

        serde_json::to_writer(outfile, &the_beastiary.beasts).unwrap();
}

