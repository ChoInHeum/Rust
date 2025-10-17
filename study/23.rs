// Tuples
// - Tuple은 ()를 사용하여 만들 수 있다.
// - () = void 아무것도 없다는 것을 의미
// <예제>
fn do_something() {}
fn do_something() -> () {}
// 위 두 코드는 같은 것을 의미한다
// 또 위 코드는 매개변수로 아무것도 받지 않고 아무것도 반환하지 않는다는 것을 의미

// Tuple은 많은 것을 가질 수 있고, 다른 type을 같이 가지고 있을 수도 있다.
// Tuple의 요소는 index를 가지고 있지만 []를 통해 접근할 수는 없지만
// .을 통해 접근할 수 있다
// <예제>
fn main() {
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );
}
// <출력>
// Inside the tuple is: First item: "Here is a name"
// Second item: 8
// Third item: ['a']
// Fourth item: 'b'
// Fifth item: [8, 9, 10]
// Sixth item: 7.7

// tuple의 type은 다음과 같다
// (&str, i32, Vec<char>, char, [i32; 3], f64)

// tuple를 사용해서 복합 변수를 만들 수 있다.
// <예제>
fn main() {
    let str_vec = vec!["one", "two", "three"];
}
// str_vec의 요소를 꺼내는 것도 tuple를 사용해서 꺼낼 수 있다.
fn main() {
    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{:?}", b);
}
// <출력>
// two

// 만약에 특정값을 꺼내는데 다른 값이 필요없을 때는 _를 사용하면 된다.
fn main() {
    let str_vec = vec!["one", "two", "three"];

    let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);
}