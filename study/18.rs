/// Mutable references
/// Mutable references를 선언하기 위해서는 &mut를 붙여 선언
fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
}

// num_ref => 주소값
// 따라서 num_ref의 value에 접근하기 위해서는 *를 사용함
fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10;
    println!("{}", my_number);

    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Second_number = triple_reference? {}", second_number == ***triple_reference);
}
// <출력>
// Second_number = triple_reference? true

// &를 사용하는 것을 referenceing 이라고 부르고
// *를 사용하는 것을 dereferencing 이라고 부른다

// mutable references 와 immutable references의 규칙
// 1. immutable references만 있을 때는 원하는 만큼 여러 개 가질 수 있음
// 2. mutable references가 있을 때는 오직 하나만 가질 수 있다.
// - 주의 mutable references와 immutable references는 동시에 가질 수 없음
