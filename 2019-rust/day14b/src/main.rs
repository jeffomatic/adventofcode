use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

lazy_static! {
    static ref ORE: String = "ORE".to_string();
    static ref FUEL: String = "FUEL".to_string();
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_quantity(s: &str) -> (String, i64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<q>\d+) (?P<mat>\w+)").unwrap();
    }

    let caps = match RE.captures(s) {
        None => panic!("invalid quantity: {}", s),
        Some(caps) => caps,
    };

    (
        caps["mat"].parse::<String>().unwrap(),
        caps["q"].parse::<i64>().unwrap(),
    )
}

#[derive(Clone, Debug)]
struct Recipe {
    output: i64,
    ingredients: Vec<(String, i64)>,
}

fn adjust_quantity(quantities: &mut HashMap<String, i64>, k: &String, q: i64) {
    let prev = *quantities.get(k).unwrap_or(&0);
    quantities.insert(k.to_string(), prev + q);
}

fn get_quantity(quantities: &HashMap<String, i64>, k: &String) -> i64 {
    *quantities.get(k).unwrap_or(&0)
}

fn redeem_byproduct(
    recipes: &HashMap<String, Recipe>,
    prev_byproduct: &HashMap<String, i64>,
) -> (i64, HashMap<String, i64>) {
    let mut prev_byproduct = prev_byproduct.to_owned();

    loop {
        let mut modified = false;
        let mut next_byproduct = prev_byproduct.to_owned();

        for (mat, q) in prev_byproduct.iter() {
            match recipes.get(mat) {
                None => (),
                Some(recipe) => {
                    if *q >= recipe.output {
                        let f = *q / recipe.output;
                        adjust_quantity(&mut next_byproduct, &mat, -(f * recipe.output));
                        for (ing, ing_amt) in recipe.ingredients.iter() {
                            adjust_quantity(&mut next_byproduct, &ing, f * *ing_amt);
                        }
                        modified = true;
                    }
                }
            }
        }

        if !modified {
            let ore = *next_byproduct.get(&*ORE).unwrap_or(&0);
            next_byproduct.remove(&*ORE);
            return (ore, next_byproduct);
        }

        prev_byproduct = next_byproduct;
    }
}

fn get_fuel_cost(recipes: &HashMap<String, Recipe>) -> (i64, HashMap<String, i64>) {
    let mut needs: HashMap<String, i64> = HashMap::new();
    needs.insert(FUEL.to_string(), 1);

    let mut byproduct = HashMap::new();

    loop {
        let mut next: HashMap<String, i64> = HashMap::new();

        for (mat, q) in needs.iter() {
            let want = *q;

            // There's no recipe for ore, so just move forward if we reach it as
            // a requirement.
            if *mat == *ORE {
                adjust_quantity(&mut next, &*ORE, want);
                continue;
            }

            let recipe = recipes.get(mat).unwrap();
            let mul = if recipe.output > want {
                1
            } else if want % recipe.output == 0 {
                want / recipe.output
            } else {
                (want / recipe.output) + 1
            };

            adjust_quantity(&mut byproduct, mat, (recipe.output * mul) - want);

            for i in recipe.ingredients.iter() {
                adjust_quantity(&mut next, &i.0, i.1 * mul);
            }
        }

        if next.len() == 1 && get_quantity(&next, &*ORE) > 0 {
            return (get_quantity(&next, &*ORE), byproduct);
        }

        needs = next;
    }
}

fn main() {
    let input = get_input();
    let mut recipes = HashMap::new();

    for line in input.lines() {
        let halves: Vec<_> = line.split(" => ").collect();
        let (material, output) = parse_quantity(halves[1]);
        let recipe = Recipe {
            output: output,
            ingredients: halves[0].split(", ").map(|s| parse_quantity(s)).collect(),
        };
        recipes.insert(material, recipe);
    }

    let (ore, byproduct) = get_fuel_cost(&recipes);
    let (excess_ore, fuel_unit_byproduct) = redeem_byproduct(&recipes, &byproduct);
    let fuel_unit_cost = ore - excess_ore;

    println!("fuel unit cost: {}", fuel_unit_cost);
    println!("fuel unit byproduct: {:?}", fuel_unit_byproduct);

    let mut ore_budget = 1_000_000_000_000;
    let mut total_fuel = 0;
    let mut byproduct = HashMap::new();

    while ore_budget > fuel_unit_cost {
        println!("total {} budget {}", total_fuel, ore_budget);

        // Spend excess ore
        let fuel_q = ore_budget / fuel_unit_cost;
        total_fuel += fuel_q;
        ore_budget %= fuel_unit_cost;

        // Add new byproduct from ore spend
        for (mat, q) in fuel_unit_byproduct.iter() {
            adjust_quantity(&mut byproduct, mat, *q * fuel_q);
        }

        // Exchange byproduct for ore
        let (excess_ore, next_byproduct) = redeem_byproduct(&recipes, &byproduct);
        ore_budget += excess_ore;
        byproduct = next_byproduct;
    }

    // With the given input, the solution produces 2267483.
    // However, by trial-and-error, the correct answer is 2267486, which is 3 higher.
    println!("fuel: {}", total_fuel);
    println!("remaining ore: {}", ore_budget);
    println!("byproduct: {:?}", byproduct);
}
