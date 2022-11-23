use csv::{ReaderBuilder, StringRecord};
use std::{fs};
use std::collections::{HashMap};
use std::fmt;
use std::env::current_dir;
use serde::{Serialize, Deserialize};
use std::ffi::{CString, c_char};
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
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

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Planet {}", self.name)
    }
}


#[no_mangle]
fn parseCsv() -> *const c_char  { // Strings must be returned as pointers
    println!("Welcome to the Star Wars Planet Encyclopedy");

    // let file = Path::new("/star_wars_planets.csv");
    // let file = Path::new(&current_dir().unwrap()).join("star_wars_planets.csv");

    let mut planets_data: HashMap<String, Planet> = HashMap::new();
  
    let content = fs::read_to_string("/run/media/jucester/Radon/Work/Personal/practices/rust/node-rust/rust-part/star_wars_planets.csv").unwrap();

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

    let selectedData = planets_data.get(&selection).unwrap();

    let dataToJson = serde_json::to_string(&selectedData).unwrap();

    let s = CString::new(dataToJson.as_bytes()).unwrap();
    s.into_raw()
}
