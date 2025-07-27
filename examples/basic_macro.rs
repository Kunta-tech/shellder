use shellder::{Container, Hooks, Hookable};

#[derive(Debug)]
#[derive(Hooks)]
pub struct MyService {
    name: String
}

impl Hooks for MyService {
    fn startup(&self) {
        println!("Starting up {:?}: {}", self, self.name);
    }

    fn run(&self) {
        println!("Running {:?}: {}", self, self.name);
    }

    fn cleanup(&self) {
        println!("Cleaning up {:?}: {}", self, self.name);
    }
}

pub struct AnotherService;

fn main() {
    let container_ref = Container::new();

    let _ =container_ref.register_lazy(|| MyService {name:"TomCat".into()});
    let _ =container_ref.register_lazy(|| AnotherService);
    
    match container_ref.resolve::<MyService>() {
        Ok(service) => service.run_hooks(),
        Err(err) => println!("Service Error: {}", err),
    }
    println!("{:#?}", container_ref);
    match container_ref.resolve::<String>() {
        Ok(obj) => println!("{}", obj),
        Err(err) => println!("Error: {}", err),
    }
}
