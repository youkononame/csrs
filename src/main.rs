use itertools::Itertools;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};
use std::collections::HashMap;
use std::{env, fmt};
use num_format::{Locale, ToFormattedString};

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Rarity {
    MilSpec,
    Restricted,
    Classified,
    Covert,
    SpecialItem,
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Rarity::MilSpec => "Mil-Spec (Blue)",
                Rarity::Restricted => "Restricted (Purple)",
                Rarity::Classified => "Classified (Pink)",
                Rarity::Covert => "Covert (Red)",
                Rarity::SpecialItem => "Rare Special Item (Gold)",
            }
        )
    }
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Wear {
    FactoryNew,
    MinimalWear,
    FieldTested,
    WellWorn,
    BattleScarred,
}

impl fmt::Display for Wear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Wear::FactoryNew => "Factory New",
                Wear::MinimalWear => "Minimal Wear",
                Wear::FieldTested => "Field-Tested",
                Wear::WellWorn => "Well-Worn",
                Wear::BattleScarred => "Battle-Scarred",
            }
        )
    }
}

impl From<f64> for Wear {
    fn from(float: f64) -> Self {
        match float {
            v if v <= 0.07 => Self::FactoryNew,
            v if v <= 0.15 => Self::MinimalWear,
            v if v <= 0.38 => Self::FieldTested,
            v if v <= 0.45 => Self::WellWorn,
            _ => Self::BattleScarred,
        }
    }
}

fn get_wear_value(rng: &mut ThreadRng) -> Wear {
    let normal = Normal::new(0.26, 0.22).unwrap();

    let float = loop {
        let v = normal.sample(rng);
        if (0.0..=1.0).contains(&v) {
            break v;
        }
    };

    float.into()
}

fn get_rarity(rng: &mut ThreadRng) -> Rarity {
    let mut roll: f64 = rng.random();
    roll *= 100.0;

    match roll {
        v if v <= 0.26 => Rarity::SpecialItem,
        v if v <= 0.90 => Rarity::Covert,
        v if v <= 4.10 => Rarity::Classified,
        v if v <= 20.08 => Rarity::Restricted,
        _ => Rarity::MilSpec,
    }
}

fn get_stattrak(rng: &mut ThreadRng) -> bool {
    rng.random_range(1..=10) == 1
}

struct Item {
    rarity: Rarity,
    wear: Wear,
    stattrak: bool,
}

impl Item {
    fn random(rng: &mut ThreadRng) -> Self {
        Self {
            rarity: get_rarity(rng),
            wear: get_wear_value(rng),
            stattrak: get_stattrak(rng),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rng = &mut rand::rng();

    let mut rarity_wear_counts: HashMap<Rarity, HashMap<Wear, (usize, usize)>> = HashMap::new();

    for _ in 0..args
        .get(1)
        .expect("Expected item count")
        .parse::<i32>()
        .unwrap()
    {
        let item = Item::random(rng);
        let rarity_group = rarity_wear_counts.entry(item.rarity).or_default();
        let wear_count = rarity_group.entry(item.wear).or_default();

        wear_count.0 += 1;
        if item.stattrak {
            wear_count.1 += 1;
        }
    }

    for rarity in rarity_wear_counts.keys().sorted() {
        println!("{rarity}");

        for wear in rarity_wear_counts[rarity].keys().sorted() {
            let count = rarity_wear_counts[rarity][wear];
            println!("{wear}: {} (of which StatTrak: {})", count.0.to_formatted_string(&Locale::en), count.1.to_formatted_string(&Locale::en));
        }

        println!();
    }
}
