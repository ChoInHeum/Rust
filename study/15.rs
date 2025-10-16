// 15. Strings


// Rust는 String 과 &str 두 가지 String types를 가지고 있다.
// 이 둘의 차이점은?
// - &str은 간단한 문자열 ex> let my_variable = "Hello, world!"
// - &str은 빠름
// - String은 더 복잡한 문자열
// - 느리지만 더 많은 함수를 가지고 있음
// - String은 포인터이므로 데이터가 heap에 저장됨

// &str과 String은 둘다 UTF-8 임 Unicode와 Emojis도 출력이 가능함
fn main() {
    let name = "서태지";
    let other_name = String::from("Adrian Fahrenheit Țepeș")
}
// String::from() 이건 &str을 String으로 쉽게 만들어 주는 것임

// str은 동적인 크기의 타입임 -> 크기가 달라질 수도 있음
fn main() {
    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.", std::mem::size_of_val("서태지")); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));
}
// <출력>
// A String is always 24 bytes. It is Sized.
// And an i8 is always 1 bytes. It is Sized.
// And an f64 is always 8 bytes. It is Sized.
// But a &str? It can be anything. '서태지' is 9 bytes. It is not Sized.
// And 'Adrian Fahrenheit Țepeș' is 25 bytes. It is not Sized.

// String을 만드는 여러 가지 방법
// - String::from("This is th string text"); 이 방법은 String을 위한 방법
// - format! 이 방법은 println!과 같음. 출력 대신 String을 만들어줌
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
// <출력>
// I am Billybrobby and I come from USA but I live in Korea.

//.into()는 여러 타입 간 변환에 쓰이는 일반 변환 메서드.
// From 트레이트가 구현돼 있으면 .into()도 자동으로 사용 가능.
// String::from("text")는 타입이 명확해서 더 직관적이지만,
// "text".into()는 컴파일러가 타입을 추론해야 함 → 때로는 모호할 수 있음.
// 즉,
// - From = 명확한 변환,
// - into() = 자동 변환 (타입 추론 필요)
fn main() {
    let my_string: String = "Try to make this a String".into();
}