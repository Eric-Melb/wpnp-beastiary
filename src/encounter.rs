use rand;
use rand::Rng;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

use beastio::*;
use monster::*;
use beastiary::*;

const LOW_DIFFICULTY_ENCOUNTER_SIZE: u8 = 2;
const MID_DIFFICULTY_ENCOUNTER_SIZE: u8 = 7;
const HIGH_DIFFICULTY_ENCOUNTER_SIZE: u8 = 15;
const NON_DIFFICULTY_ENCOUNTER_SIZE: u8 = 0;

const EASY: &str = "easy";
const MEDIUM: &str = "medium";
const HARD: &str = "hard";

pub fn generate_encounter(the_beastiary: &mut Beastiary)
{
        let working_encounter = generate_potential_encounter(the_beastiary);

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

                for i in 0..(self.groups.len() - 1)
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
                for i in 0..(encounter_to_add.groups.len())
                {
                        self.groups.push
                        (
                                (encounter_to_add.groups[i].0, encounter_to_add.groups[i].1.clone())
                        );
                }
        }
}


fn generate_potential_encounter(the_beastiary: &mut Beastiary) -> PotentialEncounter
{
        let number_of_easy_monsters = find_number_of_easy_monsters();

        let complexity = get_complexity();

        let hidden_difficulty = get_hidden_difficulty();

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
        // generate a potential pool of easy monsters
        let suitably_challenging_monsters: Vec<Monster> =
                list_suitably_challenging_monsters(the_beastiary,
                                                   EASY,
                                                   complexity);

        potential_encounter_builder(&suitably_challenging_monsters, number_of_easy_monsters)
}

fn generate_mixed_monsters(the_beastiary: &mut Beastiary, number_of_easy_monsters: u8, complexity: u8) -> PotentialEncounter
{
        let mut rng = rand::thread_rng();
        let mut easy_monsters = generate_easy_monsters(the_beastiary,
                                                        number_of_easy_monsters,
                                                        complexity);
        let mut number_of_threes: u8 = 0;
        let mut names_to_remove: Vec<String> = Vec::new();


        // keep generating encounters filled with easy monsters until we have one that can validly be replaced with at least one medium monster
        loop
        {
                // find the number of groups that have at least 3 monsters (i.e. could be replaced by a medium)
                for (amount, monster_type) in &easy_monsters.groups
                {
                        if *amount > 3
                        {
                                number_of_threes = number_of_threes + 1;
                                names_to_remove.push(monster_type.name.clone());
                        }
                }

                if number_of_threes != 0
                {
                        break;
                }

                // regenerate encounter if there's no 3s to replace [TODO: this will loop infinitely for 2 players in a non-combat party]
                easy_monsters = generate_easy_monsters(the_beastiary, number_of_easy_monsters, complexity);
        }


        let points_to_replace = rng.gen_range(0, number_of_threes);

        // remove between 1 and number_of_threes groups (that can be validly removed)
        for i in 0..points_to_replace as usize
        {
                // monsters are already in a random order in the potential encounter so we can just iterate here
                easy_monsters.remove(&names_to_remove[i]);
        }

        // add in the replacement medium monsters
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
        let hard_solos = list_suitably_challenging_monsters(the_beastiary, HARD, complexity);

        potential_encounter_builder(&hard_solos, 1)
}

fn potential_encounter_builder(monsters: &Vec<Monster>, budget: u8) -> PotentialEncounter
{
        let mut rng = rand::thread_rng();

        let mut groups: Vec<(u8, Monster)> = Vec::new();
        let mut current_monster_points = 0;
        let monster_points_cap = budget;
        let number_unique_monsters = monsters.len();


        while monster_points_cap - current_monster_points > 2
        {
                //let current_pick = rng.gen_range(0, number_unique_monsters);
                let current_pick = 1; //TODO: the rng above is fucked?

                let org: &MonsterOrg = &monsters[current_pick].organisation;

                if org.max == (monster_points_cap - current_monster_points)
                {
                        groups.push((org.max, monsters[current_pick].clone()));
                        current_monster_points = current_monster_points - org.max;
                }
                else if org.max < (monster_points_cap - current_monster_points)
                {
                        let number = rng.gen_range(org.min, org.max);
                        groups.push((number, monsters[current_pick].clone()));
                        current_monster_points = current_monster_points - number; // TODO: this line's fucked
                }
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
        // nc - 0, low -
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

        let _low = read_first_char("L".to_string());
        let _mid = read_first_char("M".to_string());
        let _high = read_first_char("H".to_string());
        let _non_combat = read_first_char("N".to_string());

        match power
        {
                _low if _low == power => LOW_DIFFICULTY_ENCOUNTER_SIZE,
                _mid if _mid == power => MID_DIFFICULTY_ENCOUNTER_SIZE,
                _high if _high == power => HIGH_DIFFICULTY_ENCOUNTER_SIZE,
                _non_combat if _non_combat == power => NON_DIFFICULTY_ENCOUNTER_SIZE,
                _ => LOW_DIFFICULTY_ENCOUNTER_SIZE
        }
}

fn get_hidden_difficulty() -> u8
{
        let mut rng = rand::thread_rng();
        let difficulty_options: [u8; 3] = [1, 2, 3];
        let weights: [u8; 3] = [2, 3, 1];
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
        let _random = read_first_char("R".to_string());
        let _simple = read_first_char("S".to_string());
        let _complex = read_first_char("C".to_string());
        let _difficult = read_first_char("D".to_string());

        let comp = read_first_char(complexity);

        match comp
        {
                _random if _random == comp => 0,
                _simple if _simple == comp => 1,
                _complex if _complex == comp => 2,
                _difficult if _difficult == comp => 3,
                _ => 0
        }
}


fn get_number_of_players() -> u8
{
        read_int("How many players will be facing this encounter") as u8
}
