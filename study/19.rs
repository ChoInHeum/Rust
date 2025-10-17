// Giving references to functions

fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Austria");
    print_country(country);
    print_country(country); // Error
}
// 두 번째 print_country에서는 country가 이미 죽었기 때문에 사용하면 Error가 남

// <해결 방안>
fn print_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name
}

fn main() {
    let country = String::from("Austria");
    let country = print_country(country);
    print_country(country);
}
// 위 코드는 실행의 문제는 없지만
// 약간 어색하다고 함 -> shadowing을 사용해서?

// <더 나은 해결 방안>
fn print_country(country_name: &String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Austria");
    print_country(&country);
    print_country(&country);
}
//