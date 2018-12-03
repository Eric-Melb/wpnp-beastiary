use std::fs::OpenOptions;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod beastio;
use beastio::*;

mod monster;
use monster::*;

fn main()
{
        // for our struct's hashmaps - these could be structs themselves but then we couldn't use "ref"
        let statistic_names = ["int", "ref", "dex", "body", "spd", "emp", "cra", "will", "luck"];
        let derived_statistic_names = ["stun", "run", "leap", "sta", "enc", "rec", "hp", "vigor"];
        let monster_statistic_names = ["height", "weight", "intelligence"];

        let skill_names = ["spell_casting", "archery", "staff_or_spear", "swordsmanship",
                "crossbow", "small_blades", "melee", "brawling", "hex_weaving", "ritual_crafting",
                "dodge_or_escape", "athletics", "awareness", "stealth", "wilderness_survival",
                "resist_magic", "resist_coercion", "endurance", "courage"];

        let mut beastiary: Vec<Monster> = Vec::new();
        //let outfile = File::create("beastiary.json").unwrap();
        // TODO: need to load the file contents into the beastiary before we start so the json doesn't get fucked up
        let outfile = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("beastiary.json")
                .unwrap();

        loop
        {
                beastiary.push(enter_monster(statistic_names, derived_statistic_names, monster_statistic_names, skill_names));

                if !keep_going()
                {
                        break;
                }

        }

        serde_json::to_writer(outfile, &beastiary).unwrap();

}

fn enter_monster(
        statistic_names: [&str; 9],
        derived_statistic_names: [&str; 8],
        monster_statistic_names: [&str; 3],
        skill_names: [&str; 19]
) -> Monster
{
        // hardcoded for now
        let book= "Witcher".to_string();

        let name = read_string("enter monster name");
        let page = read_int("enter page number");

        let mut threat: HashMap<String, String> = HashMap::new();
        let threat_difficulty = read_string("enter threat difficulty");
        threat.insert("difficulty".to_string(), threat_difficulty);
        let threat_complexity = read_string("enter threat complexity");
        threat.insert("complexity".to_string(), threat_complexity);

        let bounty = read_int("enter bounty value");

        let armor = read_int("enter armor value");

        let mut statistics: HashMap<String, u8> = HashMap::new();
        for stat in statistic_names.iter()
        {
                let stat_key = stat.to_string();
                let message = format!("enter {} stat value", stat);
                let stat_value = read_int(&message) as u8;
                statistics.insert(stat_key, stat_value);
        }

        let mut derived_statistics: HashMap<String, u8> = HashMap::new();
        for stat in derived_statistic_names.iter()
        {
                let stat_key = stat.to_string();
                let message = format!("enter {} derived stat value", stat);
                let stat_value: u8 = read_int(&message) as u8;
                derived_statistics.insert(stat_key, stat_value);
        }

        let mut monster_statistics: HashMap<String, String> = HashMap::new();
        for stat in monster_statistic_names.iter()
        {
                let stat_key = stat.to_string();
                let message = format!("enter {} monster stat value", stat);
                let stat_value = read_string(&message);
                monster_statistics.insert(stat_key, stat_value);
        }

        let org_descriptor = read_string("Organisation - Enter descriptor");
        let org_min = read_int("Organisation - Enter minimum number") as u8;
        let org_max = read_int("Organisation - Enter MAXIMUM number") as u8;
        let organisation = MonsterOrg::new(org_descriptor, org_min, org_max);

        let environment = read_vec_of_strings("enter environments");
        let tags = read_vec_of_strings("enter tags");
        let vulnerabilities = read_vec_of_strings("enter vulnerabilities");
        let abilities = read_vec_of_strings("enter abilities");

        let mut skills: HashMap<String, u8> = HashMap::new();
        for skill in skill_names.iter()
        {
                let skill_key = skill.to_string();
                let message = format!("enter {} skill value", skill);
                let skill_value = read_int(&message) as u8;
                skills.insert(skill_key, skill_value);
        }

        let number_of_weapons = read_int("how many weapons/attacks does this monster have?");
        let mut weapons: Vec<Weapon> = Vec::new();
        for i in 0..number_of_weapons
        {
                println!("Enter stats for weapon number {}", i + 1);
                let w_name = read_string("enter weapon name");
                let w_damage = read_string("enter weapon damage");
                let w_effect = read_vec_of_strings("enter effect names");
                let w_rof = read_int("enter rate of fire") as u8;
                weapons.push(Weapon::new(w_name, w_damage, w_effect, w_rof));
        }

        let loot = read_vec_of_strings("enter loot");

        Monster::new
        (
                name,
                book,
                page,
                threat,
                bounty,
                armor,
                statistics,
                derived_statistics,
                monster_statistics,
                organisation,
                environment,
                tags,
                vulnerabilities,
                abilities,
                skills,
                weapons,
                loot
        )
}
