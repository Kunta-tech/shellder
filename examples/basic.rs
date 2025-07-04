// Copyright (c) 2025 Saugata Kundu
// Licensed under the Apache-2.0 License

use mycelium::CONTAINER;

#[derive(Debug)]
struct Config {
    db_url: String,
}
#[derive(Debug)]
struct DataBase {
    name: String,
}

fn main() {
    CONTAINER.register(Config {
        db_url: "postgres://localhost".into(),
    }).expect("Failed to register Config");

    match CONTAINER.resolve::<Config>() {
        Ok(cfg) => println!("Got config: {:?}", cfg),
        Err(e) => eprintln!("Error resolving Config: {}", e),
    }

    CONTAINER.register_lazy(|| {
        println!("Lazy Init...");
        DataBase {
            name: "pokemon".into(),
        }
    }).expect("Failed to register Config");

    match CONTAINER.resolve::<DataBase>() {
        Ok(db) => println!("Got config: {:?}", db),
        Err(e) => eprintln!("Error resolving Config: {}", e),
    }
}
