use emerald::*;
use nano_ogmo::*;

pub struct Example {}
impl Game for Example {
    fn initialize(&mut self, mut emd: Emerald) {
        let ogmo_json = emd.loader().string("./assets/nano.ogmo").unwrap();
        println!("{:?}", Project::from_json(&ogmo_json).unwrap());
    }
}

fn main() {
    emerald::start(Box::new(Example { }), GameSettings::default())
}