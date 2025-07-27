// Copyright (c) 2025 Saugata Kundu
// Licensed under the Apache-2.0 License


use shellder::DEFAULT_CONTAINER;

#[derive(Debug)]
struct Config {
    db_url: String,
}
#[derive(Debug)]
struct DataBase {
    name: String,
}

fn main() {
    DEFAULT_CONTAINER.register(Config {
        db_url: "postgres://localhost".into(),
    }).expect("Failed to register Config");

    match DEFAULT_CONTAINER.resolve::<Config>() {
        Ok(cfg) => println!("Got config: {:?}", cfg),
        Err(e) => eprintln!("Error resolving Config: {}", e),
    }

    DEFAULT_CONTAINER.register_lazy(|| {
        println!("Lazy Init...");
        DataBase {
            name: "pokemon".into(),
        }
    }).expect("Failed to register Config");

    match DEFAULT_CONTAINER.resolve::<DataBase>() {
        Ok(db) => println!("Got config: {:?}",  db),
        Err(e) => eprintln!("Error resolving Config: {}", e),
    }

}
