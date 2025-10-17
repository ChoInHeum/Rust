// Vectors
// - Vector는 Array보다는 느리지만 Array보다 더 많은 함수를 가지고 있음
// - Vector는 Vec로 사용함

// <예제>
fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();

    my_vec.push(name1);
    my_vec.push(name2);
}
// 주의: Vec::new() 통해 Vector를 선언하고 값을 넣지 않으면
// compiler가 Error를 발생시킴 -> Vector의 type을 정할 수 없기 때문
// 따라서 먼저 type을 저정해 주거나 값을 꼭 넣어줘야 함

fn main() {
    let mut my_vec: Vec<String> = Vec::new();
}
// Vector 또한 반드시 같은 type의 값을 넣어야 한다.

// Vector를 쉽게 선언하는 방법은 macro를 사용하는 것이다.
fn main() {
    let mut my_vec = vec![8, 10, 10]; // Array처럼 보일 수 있지만 Vector임
}

// Vec<Vec<String>> String Vector의 Vector <- 이렇게 선언할 수도 있다.

// Vectro도 slicing을 할 수 있다.
// <예제>
fn main() {
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("Three to five: {:?},
start at two: {:?}
end at five: {:?}
everything: {:?}", three_to_five, start_at_two, end_at_five, everything);
}

// Vector는 capacity라는 개념이 있음
// capacity는 미리 확보된 메모리 공간의 크기라는 뜻임
// 만약, push를 통해 값을 추가 하면 capacity에 점점 가까워지고
// 초과할 경우, capacity를 두 배로 늘림
// 두 배로 늘리는 과정을 reallocation이라고 함
// .capacity() 매소드를 사용해서 용량이 어떻게 변하는지 알 수 있음
// <예제>
fn main() {
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // 1 element: prints 4. Vecs with 1 item always start with capacity 4
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // 4 elements: still prints 4.
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8. We have 5 elements, but it doubled 4 to 8 to make space
}
// <출력>
// 0
// 4
// 4
// 8

// Vector를 생성할 때, 미리 capacity를 줄 수 도 있음
// <예제>
fn main() {
    let mut num_vec = Vec::with_capacity(8); // Give it capacity 8
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8.
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more // Now we have 5 elements
    println!("{}", num_vec.capacity()); // Still 8
}
// 결론적으로, Vec:with_capacity(n) 을 이용하면 reallocation이 필요없기 때문에
// Vector를 더 효율적으로 관리할 수 있다.

// .into()를 사용하여 &str -> String과 같이 Array -> Vec도 변환할 수 있다. 결국 이것도 type inferences임
// Vec<_>를 사용하여 Rust의 type inferences를 사용할 수도 있다.
fn main() {
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into(); // Vec<_> means "choose the Vec type for me"
                                             // Rust will choose Vec<i32>
}
