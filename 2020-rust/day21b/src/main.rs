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

    let mut ingredients_by_allergen: HashMap<String, HashSet<String>> = HashMap::new();
    for r in recipes.iter() {
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

    // Each entry is a pair of
    // - an ingredients to delete from other lists
    // - the allergen that the ingredient conttains
    let mut q: Vec<(String, String)> = ingredients_by_allergen
        .iter()
        .filter(|(_, ings)| ings.len() == 1)
        .map(|(a, ings)| (ings.iter().nth(0).unwrap().clone(), a.clone()))
        .collect();
    while let Some((ingredient_to_remove, allergen_to_skip)) = q.pop() {
        for (other_allergen, other_ingredients) in ingredients_by_allergen.iter_mut() {
            if *other_allergen == allergen_to_skip {
                continue;
            }

            if other_ingredients.remove(&ingredient_to_remove) && other_ingredients.len() == 1 {
                q.push((
                    other_ingredients.iter().nth(0).unwrap().clone(),
                    other_allergen.clone(),
                ));
            }
        }
    }

    let mut pairs: Vec<(String, String)> = ingredients_by_allergen
        .iter()
        .map(|(allergen, ings)| (allergen.clone(), ings.iter().nth(0).unwrap().clone()))
        .collect();
    pairs.sort_by(|a, b| String::cmp(&a.0, &b.0));

    println!(
        "{}",
        pairs
            .iter()
            .map(|(_, ing)| ing.clone())
            .collect::<Vec<String>>()
            .join(",")
    );
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
