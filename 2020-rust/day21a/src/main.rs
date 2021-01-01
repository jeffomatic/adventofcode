use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Recipe {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^(.*) \(contains (.*)\)$"#).unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(From::from(format!("{}", s))),
            Some(caps) => caps,
        };

        return Ok(Recipe {
            ingredients: caps[1].split(" ").map(|s| s.to_string()).collect(),
            allergens: caps[2].split(", ").map(|s| s.to_string()).collect(),
        });
    }
}

fn main() {
    let recipes: Vec<Recipe> = get_input()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut allergens: HashSet<String> = HashSet::new();
    let mut ingredients: HashSet<String> = HashSet::new();
    let mut ingredients_by_allergen: HashMap<String, HashSet<String>> = HashMap::new();
    for r in recipes.iter() {
        ingredients.extend(r.ingredients.iter().cloned());
        allergens.extend(r.allergens.iter().cloned());

        for a in r.allergens.iter() {
            if let Some(ings_for_allergen) = ingredients_by_allergen.get_mut(a) {
                *ings_for_allergen = ings_for_allergen
                    .intersection(&r.ingredients)
                    .cloned()
                    .collect();
            } else {
                ingredients_by_allergen.insert(a.clone(), r.ingredients.clone());
            }
        }
    }

    let mut all_possible_allergic_ings: HashSet<String> = HashSet::new();
    for (_, ings) in ingredients_by_allergen.iter() {
        all_possible_allergic_ings = all_possible_allergic_ings.union(ings).cloned().collect();
    }

    let nonallergic_ings: HashSet<String> = ingredients
        .difference(&all_possible_allergic_ings)
        .cloned()
        .collect();

    let re = recipes.iter().fold(0, |accum, r| {
        accum + nonallergic_ings.intersection(&r.ingredients).count()
    });
    println!("{}", re);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
