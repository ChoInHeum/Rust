/// 12. Mutability (changing)
/// 변수를 선언할 때 사용하는 let은 변수를 immutable로 선언하는 것임
fn main() {
    let my_number = 8;
    my_number = 10; // Error my_number is immutable
}

// 변수를 선언하고 변경 및 수정을 하기 위해서 mut 수식어를 붙여야 함
fn main() {
    let mut my_number = 8;
    my_number = 10;
}
// 변수의 값은 수정이 가능하지만 변수의 type를 변경하는 것은 불가능

/// 생략
/// 변수를 선언할 때, mut를 생략하는 것은 immutable 변수로 선언한다는 의미
fn main() {
    let my_number = 8;
    println!("{}", my_number);
    {
        // 다른 code block에 있으면 같은 이름의 변수더라도 다른 변수로 취급
        let my_number = 9.2;    // 이렇게 가리는 것을 shaowing이라고 함 이전 변수가 없어지는 것이 아니라 새로운 변수를 만드는 것
        println!("{}", my_number)
    }
    println!("{}", my_number);  // 출력: 8
}