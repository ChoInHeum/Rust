fn main() {
    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";
    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );
    println!("{}", together);
}