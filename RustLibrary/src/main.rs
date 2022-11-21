#![allow(warnings)]
trait Animal {
    fn get_kind(&self) -> String;
}
 
struct Dog {}
 
impl Animal for Dog {
    fn get_kind(&self) -> String {
        "Dog".to_string()
    }
}
 
struct Cat {}
 
impl Animal for Cat {
    fn get_kind(&self) -> String {
        "Cat".to_string()
    }
}
 
// 静态分发
fn get_animal_static() -> impl Animal {
    let x = 0;
    if x == 1 {
        Dog {}
    } else {
        Dog {}
    }
}
 
// 动态分发
fn get_animal_dyn() -> Box<dyn Animal> {
    let x = 0;
    if x == 1 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}
 
fn main() {
    let animal = get_animal_static();
    println!("{}", animal.get_kind());
 
    let animal = get_animal_dyn();
    println!("{}", animal.get_kind());
}