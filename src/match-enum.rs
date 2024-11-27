enum Country {
    Australia,
    Japan,
}

enum City {
    Perth(Country),
    Tokyo(Country),
}

fn main() {
    let perth = City::Perth(Country::Australia);
    let tokyo = City::Tokyo(Country::Japan);

    match tokyo {
        City::Perth(Country::Australia) => println!("Match"),
        _ => println!("No match"),
    }
}
