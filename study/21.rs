// Collection types
// Array
// - Array는 Square brackets 안의 데이터임
// - size를 변경할 수 없고
// - 무조건 같은 type으로만 구성해야 함
// - 빠름

// 예제
fn main() {
    let array1 = ["One", "Two"];            // this array type = [&str; 2]
    let array2 = ["One", "Two", "Five"];    // this array type = [&str; 3]
}
// 위 코드의 두 array는 type이 다름!

// 변수의 type을 모를 때는 compiler한테 물어보면 됨
fn main() {
    let seasons = ["Spring", "Summer", "Autumn", "Winter"];
    let seasons2 = ["Spring", "Summer", "Fall", "Autumn", "Winter"];
    seasons.ddd(); // Error
    seasons2.thd(); // Error
}
// => 일부로 Error 일으켜 compiler가 출력하는 Error 메시지의 type을 보면 됨

// Array 안에 같은 값을 넣어서 선언할 경우 다음과 같이 선언할 수 있음
fn main() {
    let my_array = ["a"; 10];
    println!("{:?}", my_array);
}
// <출력>
// ["a", "a", "a", "a", "a", "a", "a", "a", "a", "a"]
// 이 방식은 buffer를 만들 때 많이 사용됨
let mut buffer = [0; 640];

// 또한 Array의 요소들은 index를 통해서 접근이 가능함
fn main() {
    let my_numbers = [0, 10, -20];
    println!("{}", my_numbers[1]);  // prints 10
}

// Array를 slicing하는 것도 됨
fn main() {
    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let three_of_five = &array_of_ten[2.. 5]; // 시작은 inclusive고 끝은 exclusive
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!("Three to five: {:?}, start at two: {:?}, end at five: {:?}, everything: {:?}", three_to_five, start_at_two, end_at_five, everything);
}

// 기억해야 할 것
// 1. index는 0으로 시작함
// 2. 처음은 inclusive고 마지막은 exclusive임

// 마지막을 inclusive로 설정할 수도 있음
// [0..=5];