use std::collections::HashMap;

#[macro_use]

#[derive(Serialize, Deserialize)]
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