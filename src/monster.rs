use std::collections::HashMap;

use beastio::*;

#[macro_use]

#[derive(Serialize, Deserialize, Clone)]
pub struct Monster
{
        pub name: String,
        pub book: String,
        pub page: u16,
        pub threat: HashMap<String, String>,
        pub bounty: u16,
        pub armor: u16,
        pub statistics: HashMap<String, u8>,
        pub derived_statistics: HashMap<String, u8>,
        pub monster_statistics: HashMap<String, String>,
        pub organisation: MonsterOrg,
        pub environment: Vec<String>,
        pub tags: Vec<String>,
        pub vulnerabilities: Vec<String>,
        pub abilities: Vec<String>,
        pub skills: HashMap<String, u8>,
        pub weapons: Vec<Weapon>,
        pub loot: Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MonsterOrg
{
        pub descriptor: String,
        pub min: u8,
        pub max: u8
}

impl MonsterOrg
{
        pub fn new(descriptor: String, min: u8, max: u8) -> MonsterOrg
        {
                MonsterOrg{descriptor, min, max}
        }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Weapon
{
        name: String,
        damage: String,
        effect: Vec<String>,
        rof: u8
}

impl Weapon
{
        pub fn new(name: String, damage: String, effect: Vec<String>, rof: u8) -> Weapon
        {
                Weapon{name, damage, effect, rof}
        }
}

impl Monster
{
        pub fn attack(&self, weapon_number: u8) -> u8
        {
                //TODO: roll damage from passed weapon
                let damage = 1;

                damage
        }

        pub fn abilities(&self) -> String
        {
                let mut abilities: String = String::new();
                //TODO: report ability names
                for i in 0..self.abilities.len()
                {
                        abilities += &self.abilities[i];
                }

                abilities
        }

        pub fn damage(&mut self, damage: u8) -> u8
        {
                //TODO: lower hp by amount and return new hp
                self.derived_statistics["hp"] -= damage;

                let current_hp = self.derived_statistics["hp"];

                current_hp
        }

}

impl Monster
{
        pub fn new
        (
                name: String,
                book: String,
                page: u16,
                threat: HashMap<String, String>,
                bounty: u16,
                armor: u16,
                statistics: HashMap<String, u8>,
                derived_statistics: HashMap<String, u8>,
                monster_statistics: HashMap<String, String>,
                organisation: MonsterOrg,
                environment: Vec<String>,
                tags: Vec<String>,
                vulnerabilities: Vec<String>,
                abilities: Vec<String>,
                skills: HashMap<String, u8>,
                weapons: Vec<Weapon>,
                loot: Vec<String>
        ) -> Monster
        {
                Monster
                {
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
                }
        }

}

pub fn enter_monster(
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