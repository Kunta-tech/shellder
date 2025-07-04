use mycelium::CONTAINER;

#[derive(Debug)]
struct Config {
    db_url: String,
}

fn main() {
    CONTAINER.register(Config {
        db_url: "postgres://localhost".into(),
    });

    let cfg = CONTAINER.resolve::<Config>().unwrap();
    println!("Resolved config: {:?}", cfg);
}
