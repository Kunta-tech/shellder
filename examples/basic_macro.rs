use shellder::{component, Container, Hooks, Registerable};
use shellder_macros::Hooks;

#[derive(Debug)]
#[derive(Hooks)]
#[component]
pub struct MyService;

impl Hooks for MyService {
    fn startup(&self) {
        println!("Starting up {:?}", self);
    }

    fn run(&self) {
        println!("Running {:?}", self);
    }

    fn cleanup(&self) {
        println!("Cleaning up {:?}", self);
    }
}

#[component]
pub struct AnotherService;

fn main() {
    let container_ref = Container::new();

    MyService::register(&container_ref);
    AnotherService::register(&container_ref);
    
    MyService::run();

}
