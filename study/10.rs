/// Printing "Hello world!"
fn main() {
    println!("Hello world!");

    // - fn은 함수를 뜻함
    // - main은 프로그램의 시작하는 함수
    // - () 매개변수가 없다는 것
    // - {} <- code block
    // - println!은 console에 출력하는 macro
    // - macro는 code를 작성하는 것과 같은 것 !를 사용함.
}

fn main() {
    println!("Hello, world numbr {}!", 8);
    // println! 안의 {}은 변수를 여기에 넣으라는 것임
    // 출력: Hello, world number 8!
    println!("Hello, worlds number {} and {}!", 8, 9);
    // 다음과 같이 여러 개 넣을 수 있음
    // 출력: Hello, world number 8 and 9!
}

/// 함수 만들기
fn number() -> i32 {
    8
}
fn main() {
    println!("Hello, world number {}!", number());
}
/// - number은 ()이기 때문에 매개변수가 없음
/// - -> (called a "skinny arrow")는 함수의 반환값을 보여줌
/// - number의 8 뒤에 ;이 없기 때문에 8을 반환
/// - 만약에 ;이 있었음 아무것도 반환하지 않음
/// - return 8;도 사용할 수 있지만 Rust에서 권장하지 않음

// 함수에 매개변수 전달하기
fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}
fn main() {
    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);
}

// multiply 함수를 출력 형태가 아닌 i32 반환 형태로 만들기
fn multiply(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
    result
}
fn main() {
    let multiply_result = multiply(8, 9);
}

// 변수 선언하기 및 code block({})
fn main() {
    let my_number = 8;
    println!("Hello, number {}!", my_number);
}
// 변수는 {} 여기 안에서 존재함
// 따라서 아래의 코드는 오류
fn main() {
    {
        let my_number = 8;
    }

    println!("Hello, number {}", my_number); // Error
}

// code block은 값을 반환하는 형태로도 사용가능
fn main() {
    let my_number = {
        let second_number = 8;
        second_number + 9
    };

    println!("My number is: {}", my_number);
}
// 만약, second_number + 9에 ;이 들어가면 my_number에는 아무것도 들어가지 않기 때문에 Error