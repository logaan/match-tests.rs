enum Country {
    Australia,
    Japan,
}

struct City {
    name: String,
    country: Country,
}

fn main() {
    let perth = City {
        name: "Perth".to_string(),
        country: Country::Australia,
    };

    let tokyo = City {
        name: "Tokyo".to_string(),
        country: Country::Japan,
    };

    match perth {
        City {
            name: _,
            country: Country::Australia,
        } => println!("Match"),
        _ => println!("No match"),
    }
}
