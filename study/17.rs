/// 17. More one references
/// Rust는 references는 메모리 접근이 안전하다는 확신하기 위해 사용한다.
fn main() {
    let country = String::from("Austria");
    let ref_one = &country;
    let ref_two = &country;

    println!("{}", ref_one);
}
// <출력>
// Austria
// 위 코드에서 ref_one 과 ref_two는 &string type이다.
// 한 개의 변수에서 여러 개의 reference를 만드는 것은 문제가 되지 않음
 
fn return_str() -> &str {
    let country = String::from("Austria");
    let country_ref = &country;
    country_ref // Error
}

fn main() {
    let country = return_str();
}
// 위 코드는 실행되지 않음
// let country는 return_str()에서만 존재하기 때문에 
// main에서 return_str()에서 생성된 변수의 주소를 받을 수 없음!