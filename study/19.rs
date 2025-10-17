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
// <출력>
// Austria
// Austria

// mutable references의 다른 예제
fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now is says: {}", country_name);
}

fn main() {
    let mut country = String::from("Austria");
    add_hungary(&mut country);
}
// <출력>
// Now it says: Austria-Hungary

// 결론
// - fn function_name(variable: String)은 String과 소유권을 가져오는 것 반환값이 없음
// - fn function_name(variable: &String)은 String과 그 값을 보는 것을 빌리는 것
// - fn function_name(variable: &mut String)은 String과 그 값을 변경하는 것을 빌리는 것

// 예제
fn main() {
    let country = String::from("Austria"); // country is not mutable, but we are going to print Austria-Hungary. How?
    adds_hungary(country);
}

fn adds_hungary(mut country: String) { // Here's how: adds_hungary takes the String and declares it mutable!
    country.push_str("-Hungary");
    println!("{}", country);
}
// 위 코드는 정상적으로 실행됨
// 이유
// - immutable로 선언했지만 소유권 자체가 adds_hungary함수 이동되는 순간 mut 가능
// 따라서 country의 소유권은 adds_hungary로 갔다가 함수가 끝나면 country는 이제 사용하지 못 함