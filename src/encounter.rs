use rand;
use rand::Rng;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

use beastio::*;
use monster::*;
use beastiary::*;
use helpers::higher_of;
use helpers::lower_of;

const LOW_DIFFICULTY_ENCOUNTER_SIZE: u8 = 2;
const MID_DIFFICULTY_ENCOUNTER_SIZE: u8 = 7;
const HIGH_DIFFICULTY_ENCOUNTER_SIZE: u8 = 15;
const NON_DIFFICULTY_ENCOUNTER_SIZE: u8 = 0;

const EASY_RATING_WEIGHT: u8 = 2;
const MEDIUM_RATING_WEIGHT: u8 = 5;
const HARDSOLO_RATING_WEIGHT: u8 = 0;

const VALUE_OF_MEDIUM_MONSTER: u8 = 3;

const EASY: &str = "easy";
const MEDIUM: &str = "medium";
const HARD: &str = "hard";

pub fn generate_encounter(the_beastiary: &mut Beastiary)
{
        let working_encounter = generate_potential_encounter(the_beastiary);


        // TODO: debug / delete
        println!("Displaying ecounter:");
        display_encounter(working_encounter);
}

pub fn display_encounter(encounter: PotentialEncounter)
{
        for i in 0..encounter.groups.len()
        {
                let number = encounter.groups[i].0;
                let monster = &encounter.groups[i].1;
                if number != 1
                {
                        println!("{} {} {}s",
                                 monster.organisation.descriptor,
                                 number,
                                 monster.name
                        );
                } else {
                        println!("One {}", monster.name)
                }
        }
}

struct Encounter
{

}

struct EncounterDetails
{

}

struct MonsterByMonsterStuff
{

}

// todo: move serde derives to Encounter struct when complete
// todo: this shouldn't all be public once fully encounter functionality is in
#[derive(Serialize, Deserialize)]
pub struct PotentialEncounter
{
        // int is the number of that monster in the group
        pub groups: Vec<(u8, Monster)>,
        // "number of easy monsters" of value
        pub current_monster_points: u8,
        pub monster_points_cap: u8

        //from here options are
        // reroll encounter
        // reroll single monster group
        // if >0 hard monsters - remove hard monster (reroll's encounter with no hard monsters)
        // if >3 easy monsters - add medium monster (adds a random medium monster
        // if >0 mid monsters - remove medium monster
        // if <cap monster points - add "free" monster
        // remove monster or group without reroll
        // add specific monster or group
        // complexity for new generations is: [complexity], change complexity


}

impl PotentialEncounter
{
        // should probably refactor this to use monster index so we don't need to run another loop
        // small vec though
        fn remove(&mut self, monster_name: &str)
        {
                let mut remove_index = 0;

                for i in 0..self.groups.len()
                {
                        if self.groups[i].1.name == monster_name.to_string()
                        {
                                remove_index = i;
                        }
                }

                self.current_monster_points = self.current_monster_points - self.groups[remove_index].0;
                self.groups.remove(remove_index);
        }

        fn merge_encounter(&mut self, encounter_to_add: &PotentialEncounter)
        {
                // add the points together
                self.current_monster_points += encounter_to_add.current_monster_points;

                // go through vector and insert every item into other vector
                // if the monster is already in the self vector, just add the u8s
                for i in 0..encounter_to_add.groups.len()
                {
                        let mut merge_index = -1;
                        let mut merge_existing = false;
                        for j in 0..self.groups.len()
                        {
                                if encounter_to_add.groups[i].1.name == self.groups[j].1.name
                                {
                                        merge_index = j as i32;
                                }
                        }
                        if merge_index >= 0
                        {
                                self.groups[merge_index as usize].0 += encounter_to_add.groups[i].0;
                        }
                        else
                        {
                                self.groups.push
                                (
                                        (encounter_to_add.groups[i].0, encounter_to_add.groups[i].1.clone())
                                );
                        }

                }
        }
}


fn generate_potential_encounter(the_beastiary: &mut Beastiary) -> PotentialEncounter
{
        let number_of_easy_monsters = find_number_of_easy_monsters();

        let complexity = get_complexity();

        let hidden_difficulty = get_hidden_difficulty();

        // hidden difficult determines prevalence of medium and solo monsters
        // based on constants at the top of this file
        match hidden_difficulty
        {
                1 => (generate_easy_monsters(the_beastiary, number_of_easy_monsters, complexity)),
                2 => (generate_mixed_monsters(the_beastiary, number_of_easy_monsters, complexity)),
                3 => (generate_hard_solo(the_beastiary, complexity)),
                _ => (generate_easy_monsters(the_beastiary, number_of_easy_monsters, complexity))
        }
}

fn list_suitably_challenging_monsters(the_beastiary: &mut Beastiary, difficulty: &str, complexity: u8) -> Vec<Monster>
{

        let mut suitable_difficulty_monsters: Vec<Monster> = Vec::new();

        for (_name, creature) in &the_beastiary.beasts
        {
                if creature.threat["difficulty"] == difficulty
                {
                        suitable_difficulty_monsters.push(creature.clone());
                }
        }
        // find monsters that meet our complexity requirements
        let mut suitably_challenging_monsters: Vec<Monster> = Vec::new();

        for creature in suitable_difficulty_monsters
        {
                if compare_complexity(&creature.threat["complexity"], complexity)
                {
                        suitably_challenging_monsters.push(creature);
                }
        }

        suitably_challenging_monsters
}

fn generate_easy_monsters(the_beastiary: &mut Beastiary, number_of_easy_monsters: u8, complexity: u8) -> PotentialEncounter
{
        println!("EASY");
        // generate a potential pool of easy monsters
        let suitably_challenging_monsters: Vec<Monster> =
                list_suitably_challenging_monsters(the_beastiary,
                                                   EASY,
                                                   complexity);

        potential_encounter_builder(&suitably_challenging_monsters, number_of_easy_monsters)
}

fn generate_mixed_monsters(the_beastiary: &mut Beastiary, number_of_easy_monsters: u8, complexity: u8) -> PotentialEncounter
{
        println!("MIXED");
        let mut rng = rand::thread_rng();

        let mut number_of_threes: u8 = 0;
        let mut names_to_remove: Vec<String> = Vec::new();


        let mut easy_monsters = generate_easy_monsters(the_beastiary,
                                                       number_of_easy_monsters,
                                                       complexity);
        // keep generating encounters filled with easy monsters until we have one that can validly be replaced with at least one medium monster
        loop
        {
                // find the number of groups that have at least 3 monsters (i.e. could be replaced by a medium)
                for (amount, monster_type) in &easy_monsters.groups
                {
                        if *amount > 3
                        {
                                number_of_threes += 1;
                                names_to_remove.push(monster_type.name.clone());
                                println!("Could remove {}", monster_type.name)
                        }
                }

                println!("Number of threes was {}", number_of_threes);

                if number_of_threes != 0
                {

                        break;
                }

                // regenerate encounter if there's no 3s to replace [TODO: this will loop infinitely for 2 players in a non-combat party]
                easy_monsters = generate_easy_monsters(the_beastiary, number_of_easy_monsters, complexity);
        }

        let mut points_to_replace = 1;

        if number_of_threes != 1
        {
                points_to_replace = rng.gen_range(1, number_of_threes);
        }

        // remove between 1 and number_of_threes groups (that can be validly removed)
        for i in 0..points_to_replace as usize
        {
                let mut delete = true;
                let target = &names_to_remove[i];
                for j in 0..easy_monsters.groups.len()
                {
                        if &easy_monsters.groups[j].1.name == target
                        {
                                if easy_monsters.groups[j].0 > VALUE_OF_MEDIUM_MONSTER
                                {
                                        delete = false;
                                        easy_monsters.groups[j].0 -= VALUE_OF_MEDIUM_MONSTER;
                                }
                        }
                }

                if delete
                {
                        easy_monsters.remove(&names_to_remove[i]);
                }

                // monsters are already in a random order in the potential encounter so we can just iterate here

        }

        // add in the replacement medium monsters
        println!("Adding {} medium monsters", points_to_replace);
        let medium_monsters =
                potential_encounter_builder(
                        &list_suitably_challenging_monsters(the_beastiary,
                                                            MEDIUM, complexity),
                        points_to_replace);

        easy_monsters.merge_encounter(&medium_monsters);

        easy_monsters
}

fn generate_hard_solo(the_beastiary: &mut Beastiary, complexity: u8) -> PotentialEncounter
{
        println!("HARD");
        let hard_solos = list_suitably_challenging_monsters(the_beastiary, HARD, complexity);

        potential_encounter_builder(&hard_solos, 1)
}

// TODO: add error handling for being passed a vec with no monsters in it
fn potential_encounter_builder(monsters: &Vec<Monster>, budget: u8) -> PotentialEncounter
{
        if monsters.len() == 0
        {
                println!("Got passed no monsters, going to panic now");
        }
        let mut rng = rand::thread_rng();

        let mut groups: Vec<(u8, Monster)> = Vec::new();
        let mut current_monster_points = 0;
        let monster_points_cap = budget;
        let number_unique_monsters = monsters.len();
        let mut current_pick: usize = 0;

        let mut remaining_points = monster_points_cap - current_monster_points;

        while remaining_points > 0
        {
                let mut new_add = true;

                println!("CURRENT ENCOUNTER:");
                println!("Current points: {}, Cap: {}, Remaining: {}",
                         &current_monster_points, &monster_points_cap, remaining_points);

                let mut current_pick: usize = 0;
                if monsters.len() == 1
                {
                        // only 1 suitable monster, pick it
                        let current_pick = 0;
                }
                // gen_range gets a little weird if the range is 0 to 1
                else if monsters.len() == 2
                {
                        current_pick = rng.gen_range(0, 4) % 2;
                }
                else
                {
                        current_pick = rng.gen_range(0, monsters.len() - 1);
                }


                let org: &MonsterOrg = &monsters[current_pick].organisation;

                // if monsters have already been added
                // check if this monster has already been added, and if so
                // add up to the org.max or remaining points, whichever is lower
                let mut number: u8 = 0;
                let mut index: usize = 0;
                if groups.len() != 0
                {

                        for i in 0..groups.len()
                        {
                                let existing_monster = &groups[i].1;

                                let existing_number = groups[i].0;

                                if &monsters[current_pick].name == &existing_monster.name
                                {


                                        new_add = false;

                                        if existing_number < existing_monster.organisation.max
                                        {
                                                let valid_group_addition_max =
                                                        existing_monster.organisation.max - existing_number;

                                                number = lower_of(valid_group_addition_max,
                                                                  remaining_points);

                                        }
                                        break;
                                }
                        }
                }


                if !new_add
                {
                        println!("Should add {}", number);
                        groups[index].0 += number;
                        current_monster_points += number;
                        println!("adding {} {}s after updating current points to {}",
                                 number, monsters[current_pick].name, current_monster_points);
                }
                else if org.max == 0
                {
                        continue
                }
                else if org.max == org.min && org.max <= remaining_points
                {

                        current_monster_points = current_monster_points + org.min;
                        println!("pushing {} {} after updating current points to {}",
                                  org.min, monsters[current_pick].name, current_monster_points);
                        groups.push((org.min, monsters[current_pick].clone()));
                }
                else if org.max == remaining_points
                {
                        current_monster_points = current_monster_points + org.max;
                        println!("pushing {} {} after updating current points to {}",
                                  org.max, monsters[current_pick].name, current_monster_points);
                        groups.push((org.max, monsters[current_pick].clone()));
                }
                else if org.min == remaining_points
                {
                        current_monster_points = current_monster_points + org.min;
                        println!("pushing {} {} after updating current points to {}",
                                  org.min, monsters[current_pick].name, current_monster_points);
                        groups.push((org.min, monsters[current_pick].clone()));

                }
                else if org.min < remaining_points
                {
                        let mut number = rng.gen_range(org.min, org.max);

                        if number > remaining_points
                        {
                                number = rng.gen_range(org.min, remaining_points);
                        }

                        current_monster_points = current_monster_points + number;
                        println!("pushing {} {} after updating current points to {}",
                                  number, monsters[current_pick].name, current_monster_points);
                        groups.push((number, monsters[current_pick].clone()));
                }

                remaining_points = monster_points_cap - current_monster_points;
                println!()
        }

        PotentialEncounter{groups, current_monster_points, monster_points_cap}
}

fn compare_complexity(creature_complexity: &str, desired_complexity: u8) -> bool
{
        // random complexity
        if desired_complexity == 0
        {
                return true;
        }

        // simple, complex, and difficult complexity
        let complexity = convert_complexity(creature_complexity.to_string());

        if complexity == desired_complexity
        {
                return true;
        }
        else
        {
                return false;
        }
}

fn find_number_of_easy_monsters() -> u8
{
        let mut rng = rand::thread_rng();
        let number_of_players = get_number_of_players();

        let party_power = get_party_power();

        let number_of_easy_monsters = number_of_players + party_power;

        //multiply by anywhere between .75 and 1.25
        let float_monsters = number_of_easy_monsters as f32 * rng.gen_range(0.75, 1.25);

        let number_of_easy_monsters = float_monsters as u8;

        number_of_easy_monsters
}

fn get_party_power() -> u8
{
        let power = read_first_char(
                read_string("Is the party facing the encounter \
                (L)ow [default], (M)id, (H)igh Power, or (N)on Combat Oriented")
                .to_uppercase()
        );

        let low = read_first_char("L".to_string());
        let mid = read_first_char("M".to_string());
        let high = read_first_char("H".to_string());
        let non_combat = read_first_char("N".to_string());

        if power == low
        {
                return LOW_DIFFICULTY_ENCOUNTER_SIZE
        } else if power == mid
        {
                return MID_DIFFICULTY_ENCOUNTER_SIZE
        } else if power == high
        {
                return HIGH_DIFFICULTY_ENCOUNTER_SIZE
        } else if power == non_combat
        {
                return NON_DIFFICULTY_ENCOUNTER_SIZE
        } else {
                return LOW_DIFFICULTY_ENCOUNTER_SIZE
        }

}

fn get_hidden_difficulty() -> u8
{
        let mut rng = rand::thread_rng();
        let difficulty_options: [u8; 3] = [1, 2, 3];
        let weights: [u8; 3] = [EASY_RATING_WEIGHT, MEDIUM_RATING_WEIGHT, HARDSOLO_RATING_WEIGHT];
        let dist = WeightedIndex::new(&weights).unwrap();
        let hidden_difficulty = difficulty_options[dist.sample(&mut rng)];

        hidden_difficulty
}

fn get_complexity() -> u8
{
        let complexity = read_string("Complexity of enemies will be (R)andom [default], (S)imple, (C)omplex, or (D)ifficult")
        .to_uppercase()
        ;

        convert_complexity(complexity)
}

fn convert_complexity(complexity: String) -> u8
{
        let random = read_first_char("R".to_string());
        let simple = read_first_char("S".to_string());
        let complex = read_first_char("C".to_string());
        let difficult = read_first_char("D".to_string());

        let comp = read_first_char(complexity.to_uppercase());

        if comp == random
        {
                return 0
        } else if comp == simple
        {
                return 1
        } else if comp == complex
        {
                return 2
        } else if comp == difficult
        {
                return 3
        } else
        {
                return 0
        }
}


fn get_number_of_players() -> u8
{
        let mut number = read_int("How many players will be facing this encounter") as u8;

        if number == 0
        {
                number = 1;
        }

        return number
}
