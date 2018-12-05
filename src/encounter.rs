use rand::Rng;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

use beastiary;
use beastio::*;

use monster;

struct Encounter
{

}

struct EncounterDetails
{

}

struct MonsterByMonsterStuff
{

}

// TODO: Number of Easy Monsters = Players + 2, never less than 3
// TODO:


struct PotentialEncounter
{
        // int is the number of that monster in the group
        groups: Vec<(u8, Monster)>,
        // "number of easy monsters" of value
        current_monster_points: u8,
        monster_points_cap: u8

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

fn get_number_of_players() -> u8
{
        read_int("How many players will be facing this encounter") as u8
}

pub fn generate_encounter(the_beastiary: &Beastiary) -> PotentialEncounter
{
        let number_of_easy_monsters = find_number_of_easy_monsters();

        let complexity = get_complexity();

        let hidden_difficulty = get_hidden_difficulty();

        match hidden_difficulty
        {
                1 => (generate_easy_monsters(the_beastiary, number_of_easy_monsters, complexity)),
                2 => (generate_mixed_monsters(the_beastiary, number_of_easy_monsters, complexity)),
                3 => (generate_hard_solo(the_beastiary, number_of_easy_monsters, complexity))
        }

}

fn generate_easy_monsters(the_beastiary: &Beastiary, number_of_easy_monsters: u8, complexity: u8) -> PotentialEncounter
{
        // generate a potential pool of easy monsters
        let mut easy_monsters: Vec<Monster> = Vec::new();

        for (name, creature) in the_beastiary.beasts
        {
                if creature.threat["difficulty"] == "easy"
                {
                        easy_monsters.push(creature);
                }
        }

        // find easy monsters that meet our complexity requirements
        let mut suitable_complexity_monsters: Vec<Monster> = Vec::new();

        for creature in easy_monsters
        {
                if compare_complexity(creature.threat["complexity"], complexity)
                {
                        suitable_complexity_monsters.push(creature);
                }
        }

        potential_encounter_builder
}

fn generate_mixed_monsters(the_beastiary: &Beastiary, number_of_easy_monsters: u8, complexity: u8) -> PotentialEncounter
{

}

fn generate_hard_solo(the_beastiary: &Beastiary, number_of_easy_monsters: u8, complexity: u8) -> PotentialEncounter
{

}

fn potential_encounter_builder(monsters: Vec<Monster>, budget: u8) -> PotentialEncounter
{
        let mut groups: Vec<(u8, Monster)> = Vec::new();
        let mut current_monster_points = 0;
        let monster_points_cap = budget;
        let number_unique_monsters = monsters.len();

        while monster_points_cap - current_monster_points > 2
        {
                let current_pick = 0;
                Rng::gen_range(current_pick, 0, number_unique_monsters);

                let org: MonsterOrg = monsters[current_pick].organisation;

                if org.max == (monster_points_cap - current_monster_points)
                {
                        groups.push((org.max, monsters[current_pick]));
                        current_monster_points = current_monster_points - org.max;
                }
                else if org.max < (monster_points_cap - current_monster_points)
                {
                        let number = 1;
                        Rng::gen_range(number, org.min, org.max);
                        groups.push((number, monsters[current_pick]));
                        current_monster_points = current_monster_points - number;
                }
        }

        PotentialEncounter{groups, current_monster_points, monster_points_cap}
}

fn compare_complexity(creature_complexity: &str, desired_complexity: u8) -> bool
{
        let c_complexity = read_first_char(creature_complexity.to_string());

        let random = read_first_char("R".to_string());
        let simple = read_first_char("S".to_string());
        let complex = read_first_char("C".to_string());
        let difficult = read_first_char("D".to_string());

        let complexity = match read_first_char(c_complexity)
        {
                random => 0,
                simple => 1,
                complex => 2,
                difficult => 3,
                _ => 0
        };

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
        let power = read_string("Is the party facing the encounter (L)ow [default], (M)id, (H)igh Power, or (N)on Combat Oriented")
                .to_upper_case()
        ;

        let low = read_first_char("L".to_string());
        let mid = read_first_char("M".to_string());
        let high = read_first_char("H".to_string());
        let non_combat = read_first_char("N".to_string());

        match read_first_char(power)
        {
                low => 2,
                mid => 5,
                high => 10,
                non_combat => 0,
                _ => 2
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
        .to_upper_case()
        ;

        let random = read_first_char("R".to_string());
        let simple = read_first_char("S".to_string());
        let complex = read_first_char("C".to_string());
        let difficult = read_first_char("D".to_string());

        match read_first_char(complexity)
        {
                random => 0,
                simple => 1,
                complex => 2,
                difficult => 3,
                _ => 0
        }
}