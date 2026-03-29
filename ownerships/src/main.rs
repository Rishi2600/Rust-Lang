struct Golem {
    name: String,
    power_source: String,
}

fn main() {
    let my_golem = Golem {
        name: String::from("Stone Sentinel"),
        power_source: String::from("Ancient Battery"),
    };

    // We MOVE the name out, but leave the power_source
    let _name_only = my_golem.name;

    // println!("{}", my_golem.name);
    println!("Power source still here: {}", my_golem.power_source);
}