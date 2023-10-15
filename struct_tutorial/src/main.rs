#[derive(Debug)]
#[derive(Clone)]

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str{
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64){
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

fn main() {
    /*
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };
     */

    let mut vehicle = Shuttle::new("Endeavour");
    println!("\nname is {}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    println!("\nvehicle is {:?}", vehicle);

    let vehicle2 = Shuttle {
        ..vehicle.clone()
    };

    println!("\nvehicle2 is {:?}", vehicle2);

    let vehicle_name = vehicle.get_name();
    println!("\nvehicle_name is {}", vehicle_name);

    println!("\npropellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.5);
    println!("propellant is {}", vehicle.propellant);
}
