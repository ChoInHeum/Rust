// 20.Copy types
// Rust는 간단한 타입은 Stack에 저장하는데
// Stack에 저장하는 types를 Copy types이라고 한다.
// Copy types는 복사하는게 쉬워서 함수로 보낼 때
// 항상 복사해서 함수로 보낸다.
// 값을 복사한 것에 대해서는 소유권에 대해 걱정할 피요 없음

// Copy types
// - integers
// - floats
// - booleans
// - char

// 예제
fn print_number(number: i32) {
    println!("{}", number);
}

fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);
}
// 출력에 문제 없음 because integers는 copy type이기 때문에
// 소유권에 대해 생각하지 않아도 됨

// Clone
// .clone()은 copy와 비슷하지만 보통 더 많은 메모리를 사용한다.
fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Kiribati");
    prints_country(country);
    prints_country(country); // ⚠️
}
// String은 copy types가 아니기 때문에 소유권에 대해 생각을 해야 함
// 하지만 .clone()을 사용하면 소유권에 대한 문제가 해결됨
fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Kiribati");
    prints_country(country.clone());
    prints_country(country);
}
// 이렇게 하면 두 번 출력할 수 있음
// String을 계속해서 .clone()을 통해 clone 하면
// 메모리 관리 측면에서 매우 비율적
// 따라서 References를 사용
fn get_length(input: &String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

fn main() {
    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words ");
        get_length(&my_string);
    }
}

// 값이 없는 변수
// - 값이 없는 변수는 초기화 되지 않은 변수라고 불림
// - 초기화 되어 있지 않다 => 아직 시작되지 않았다
fn main() {
    let my_variable; // Error
}
// 이렇게는 사용할 수 없다.
// Rust가 최기화 되지 않은 변수는 compile하지 않기 때문

// 하지만 다음과 같이 사용할 수 있다.
fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

fn main() {
    let my_number;
    {
        let number = {
            57
        };

        my_number = loop_then_return(number);
    }

    println!("{}", my_number);
}
// <출력>
// 100

fn main() {
    let my_number;
    {
        my_number = 100;
    }

    println!("{}", my_number);
}
// 위 코드는 main안의 code block안에서 변수에 값이 할당되므로 실행의 문제가 없음