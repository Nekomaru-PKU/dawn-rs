use dawn_rs::*;

fn main() {
    let instance = Instance::new(None).unwrap();
    println!("{:?}", Instance::get_capabilities());
    println!("{:?}", instance.get_wgsl_language_features().as_deref());
}
