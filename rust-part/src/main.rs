// use rust_part::sum;
use csv::{ReaderBuilder, StringRecord};
use std::{fs};
use std::collections::{HashMap};

#[derive(Debug)]
struct Planet {
    name: String,
    rotation_period: i32,
    orbital_period: i32,
    diameter: i32,
    climate: String,
    gravity: String,
    terrain: String,
    surface_water: i32,
    population: i128,

}

impl Planet {
    pub fn new(row: StringRecord) -> Self {
        Self {
            name: row.get(0).unwrap().trim().to_string(),
            rotation_period: row.get(1).unwrap().trim().parse().unwrap_or(0),
            orbital_period: row.get(2).unwrap().trim().parse().unwrap_or(0),
            diameter: row.get(3).unwrap().trim().parse().unwrap_or(0),
            climate: row.get(4).unwrap().trim().to_string(),
            gravity: row.get(5).unwrap().trim().to_string(),
            terrain: row.get(6).unwrap().trim().to_string(),
            surface_water: row.get(7).unwrap().trim().parse().unwrap_or(0),
            population: row.get(8).unwrap().trim().parse().unwrap_or(0),
        }
    }
}


fn main() {
    println!("Welcome to the Star Wars Planet Encyclopedy");
    // println!("{}", sum(2, 3));

    let mut planets_data: HashMap<String, Planet> = HashMap::new();
    
   
    let content = fs::read_to_string("star_wars_planets.csv").unwrap();

    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(content.as_bytes());

    for result in rdr.records() {
        let res = result.unwrap();
        let data = Planet::new(res);

        planets_data.insert(data.name.clone(), data);
    }


    println!("What planet info do you want?");


    let mut select = String::new();
    std::io::stdin().read_line(&mut select);

    let selection: String = select.trim().parse().unwrap();

    let mut selectedData = planets_data.get(&selection);

    println!("{:?}", selectedData);
    
}

