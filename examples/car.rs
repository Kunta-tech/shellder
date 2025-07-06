// use shellder::Container;

use shellder::{component, Container};

#[derive(Debug)]
#[component(Car)]
struct SUV {
    name: &'static str,
}
#[derive(Debug)]
#[component(Car)]
struct Sudan {
    name: &'static str,
}
trait Car {
    fn horn(&self);
}
impl Car for SUV {
    fn horn(&self) {
        println!("horning by {:?}-{}", self, self.name);
    }
}
impl Car for Sudan {
    fn horn(&self) {
        println!("horning by {:?}-{}", self, self.name);
    }
}

fn main(){
    let container_ref = Container::new();

    container_ref.register_lazy(|| SUV{name:"Maruti"}).expect("SUV not init");
    container_ref.register(Sudan {name: "Honda"}).expect("Sudan not init");

    match container_ref.resolve::<SUV>() {
        Ok(ob) => ob.horn(),
        Err(e) => println!("{}", e),
    }
}