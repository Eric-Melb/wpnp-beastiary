use std::collections::HashMap;

#[macro_use]

#[derive(Serialize, Deserialize)]
pub struct Monster
{
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
}

#[derive(Serialize, Deserialize)]
pub struct MonsterOrg
{
        descriptor: String,
        min: u8,
        max: u8
}

impl MonsterOrg
{
        pub fn new(descriptor: String, min: u8, max: u8) -> MonsterOrg
        {
                MonsterOrg{descriptor, min, max}
        }
}

#[derive(Serialize, Deserialize)]
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