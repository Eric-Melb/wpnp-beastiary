use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use serde_json;

use monster::*;


#[derive(Serialize, Deserialize)]
pub struct Beastiary
{
        // the string is their name
        pub beasts: HashMap<String, Monster>
}

// TODO: refactor so that new() spawns an empty beastiary unless a hashmap of monsters was passed
// TODO: refactor so that load_beasts() loads a file, calls new, and renames the file
impl Beastiary
{
        fn load_beasts(beastiary_file: &str) -> Beastiary
        {
                let mut beastfile = File::open(beastiary_file).unwrap();

                let mut prior_beasts = String::new();
                beastfile.read_to_string(&mut prior_beasts).unwrap();

                let beastmap: HashMap<String, Monster> = serde_json::from_str(&prior_beasts).unwrap();

                Beastiary{beasts: beastmap}
        }

        pub fn new(beastiary_file: &str, old_beastiary_file: &str) -> Beastiary
        {
                let beasts = Beastiary::load_beasts(beastiary_file);
                // "backup" beastiary, replacing any previous backup
                fs::rename(beastiary_file, old_beastiary_file).unwrap();

                beasts
        }

        pub fn check_beasts(&self)
        {
                for (name, details) in &self.beasts
                {
                        println!("{} has {} health", name, details.derived_statistics["hp"]);
                }
        }

        pub fn add_beast(&mut self, new_beast: Monster)
        {
                self.beasts.insert(new_beast.name.clone(), new_beast);
        }

}
