// use shellder::Container;

use std::sync::Arc;

use shellder::{inject, Injection, Container};

#[derive(Debug)]
struct Engine {
    name: &'static str,
}

#[inject]
#[derive(Debug)]
struct SUV {
    engine: Arc<Engine>,
}

fn main(){
    let container_ref = Container::new();

    container_ref.register_lazy(|| Engine { name: "V16"}).expect("Error V16:");
    // let _ = container_ref.register(SUV { engine: container_ref.resolve::<Engine>().expect("Error") });
    container_ref.register(SUV::inject(&container_ref)).expect("Error SUV:");

    match container_ref.resolve::<SUV>() {
        Ok(ob) => println!("{:?}", ob),
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("{:#?}", container_ref);

    
}