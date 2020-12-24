extern crate aoc2020;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use aoc2020::utils;
use regex::Regex;
use std::collections::{HashMap};
use itertools::Itertools;

fn parse(line: &String) -> (Vec<String>, Vec<String>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z ]+) \(contains ([a-z, ]+)\)$").unwrap();
    }

    let cap = RE.captures_iter(line).next().unwrap();

    let ingredients: Vec<String> = cap[1].split(" ").map(|s| String::from(s.trim())).collect();
    let allergens: Vec<String> = cap[2].split(",").map(|s| String::from(s.trim())).collect();

    (ingredients, allergens)
}

// Returns true if we found anything
fn find_and_remove_known_ingredients(allergen_map: &mut HashMap<String, Vec<String>>, allergens_with_known_ingredients: &mut HashMap<String, String>) -> bool {
    // For each allergen, look through the list of ingredients and see if there's one ingredient
    // that occurs the most times. If so, this means that every time the allergen was listed
    // that particular ingredient was listed, so it must be the ingredient. Save these ingredients
    // to a list. If there's a tie, we can't know for sure.

    // Map from allergen to ingredient that we know for sure includes the ingredient
    let mut found_new_allergens: bool = false;

    let previous_allergens_with_known_ingredients = allergens_with_known_ingredients.clone();

    allergen_map.iter()
        // Skip allergens we already know the ingredient for
        .filter(|(allergen, _)| previous_allergens_with_known_ingredients.keys().find(|k| k == allergen).is_none())
        .for_each(|(allergen, ingredients)| {
            // Count the times each ingredient occurs in the list
            let mut ingredient_counter: HashMap<&String, usize> = HashMap::new();
            ingredients.iter()
                // Skip ingredients we already know the allergen for
                .filter(|ingredient| previous_allergens_with_known_ingredients.values().find(|v| v == ingredient).is_none())
                .for_each(|i| {
                    let entry = ingredient_counter.entry(i).or_insert(0);
                    *entry += 1;
                });

            // Find the top ingredient and how many times it occurs
            let top_entry = ingredient_counter.iter()
                .max_by(|x, y| x.1.cmp(y.1)).unwrap();

            // See if theres only one ingredient with that count
            let num_extries_with_max_value = ingredient_counter.values().filter(|v| *v == top_entry.1).count();
            if num_extries_with_max_value == 1 {
                // Found it
                allergens_with_known_ingredients.insert(allergen.clone(), top_entry.0.clone().clone());
                found_new_allergens = true;
            }
        });

    found_new_allergens
}

fn find_allergens(data: Vec<String>) -> (usize, String) {
    let mut allergen_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut allergens_with_known_ingredients: HashMap<String, String> = HashMap::new();
    let mut original_ingredients_list: Vec<String> = vec![];

    // Parse the data into a map of allergens back to a list of possible ingredients. If the
    // allergen is listed multiple times then concat together all the ingredients
    data.iter().map(|l| parse(l)).for_each(|(ingredients, allergens)| {
        original_ingredients_list.append(&mut ingredients.clone());

        allergens.iter().for_each(|a| {
            allergen_map.entry(a.clone()).or_insert(vec![]).append(&mut ingredients.clone())
        });
    });

    loop {
        if !find_and_remove_known_ingredients(&mut allergen_map, &mut allergens_with_known_ingredients) {
            break;
        }
    }

    let unique_unknown_ingredients: Vec<&String> = original_ingredients_list.iter()
        .filter(|ingredient| allergens_with_known_ingredients.values().find(|i| i == ingredient).is_none())
        .collect();

    let scary_ingredients = allergens_with_known_ingredients.into_iter()
        .sorted_by_key(|(allergen, _)| allergen.clone());

    let scary_ingredients_string = scary_ingredients
        .map(|(_, ingredient)| ingredient)
        .join(",");

    (unique_unknown_ingredients.len(), scary_ingredients_string)
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/21.txt");

    let (num_unknown_ingredients, scary_ingredients_string) = find_allergens(data);

    println!("Part 1 {:?}", num_unknown_ingredients);
    println!("Part 2 {:?}", scary_ingredients_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single() {
        let (ingredients, allergens) = parse(&String::from("abc (contains x)"));
        assert_eq!(ingredients, vec![String::from("abc")]);
        assert_eq!(allergens, vec![String::from("x")]);
    }

    #[test]
    fn test_parse_multiple() {
        let (ingredients, allergens) = parse(&String::from("abc def (contains x, y)"));
        assert_eq!(ingredients, vec![String::from("abc"), String::from("def")]);
        assert_eq!(allergens, vec![String::from("x"), String::from("y")]);
    }

    #[test]
    fn test_example() {
        let data = vec![
            String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
            String::from("sqjhc fvjkl (contains soy)"),
            String::from("sqjhc mxmxvkd sbzzf (contains fish)"),
        ];

        let (num_unknown_ingredients, scary_ingredients_string) = find_allergens(data);

        assert_eq!(num_unknown_ingredients, 5);
        assert_eq!(scary_ingredients_string, "mxmxvkd,sqjhc,fvjkl");
    }
}