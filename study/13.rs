/// 13. The stack, the heap, and pointers
/// stack과 heap은 컴퓨터의 메모리 공간이다.
/// stack과 heap의 차이점
/// - stack은 매우 빠르지만 heap은 그렇지 않다. 
/// - 그렇다고 heap이 그렇게 느린 것은 아니다.
/// - stack으로 메모리의 전영역을 사용하지 않는 이유
/// - Rust compile time에 변수의 크기를 필요로 한다.
/// - 예를 들어 i32 같은 경우에는 크기가 명확하기 때문에 stack에 할당하지만
/// - 몇몇의 다른 type은 compiler time에 크기를 정확하게 할 수 없기 때문에 
/// - heap에 data를 저장하고 stack에는 heap의 해당 type의 pointer를 넣는다.
/// - type의 pointer는 정확한 크기를 알 수 있기 때문에 stack에 저장할 수 있다.
/// 
/// Pointer
/// - Rust에서 pointer는 reference라고 불림
/// - pointer는 또 다른 값의 메모리 주소를 가리키는 참조임
/// - reference는 해당 변수의 앞에 &를 붙여 사용
fn main() {
    let my_variable = 8;
    let my_reference = &my_variable;
}

// reference의 reference -> 이중 포인터?
fn main() {
    let my_number = 15; // This is an i32
    let single_reference = &mynumber; // This is a &i32
    let double_reference = &single_reference; // This is a &&i32
    let five_refererence = &&&&&my_number; // This is a &&&&&i32
}