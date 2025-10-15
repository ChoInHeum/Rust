fn type_inference() {
    /// Type inference 란?
    /// - compiler가 type을 스스로 판단하게 하는 것
    /// - 즉, 내가 타입을 직접 지정하지 않도 compiler가 알아서 결정
    /// - compiler는 항상 변수의 타입을 알고 있음
    /// - 하지만, 내가 특정 타입으로 지정앟고 싶을 때는 직접 명시해야 함
    /// 
    /// Compiler에게 type을 명시해야 하는 경우
    /// - 코드가 너무 복잡해서 compiler가 어떤 타입인지 추측하지 못할 때
    /// - 기본 타입(i32)이 아닌 다른 integer type(i128, u8) 등을 쓰고 싶을 때



    // type을 명시하는 방법

    // 1. 일반적인 타입 명시 방법
    let samll_number: u8 = 10;

    // 2. 숫자 뒤에 타입을 명시하는 방법
    let small_number = 10u8;

    // 3. _를 사용해 타입을 명시하는 방법
    let small_number = 10_u8;

    // _는 단순 타입을 지정할 뿐만이 아니라 코드의 가독성을 높이는 것에도 사용
    let small_number = 10_u8;
    let big_number = 100_00_00_i32;
    // => _는 단지 숫자를 읽기 쉽게 만드는 용도
    // => 실제 값에는 아무 영향 없음
    // => 여러 번 써도 문제 없음



    /// Floats
    /// - Rust가 float으로 인식하는 기준은 숫자 뒤의 .(dot)
    /// ex> 5. => float으로 인식
    let my_float = 5.0;

    /// float type 종류
    /// Rust에는 f32, f64 두 가지 float type이 있음
    /// 기본적으로 Rust는 f64를 사용

    // 서로 다른 float type 끼리 연산 X
    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;

    let third_float = my_float + my_other_float; // Error
    
    // 해결 방법(1) - Casting(as)
    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;

    let third_float = my_float + my_other_float as f64;
    // my_other_float를 f64로 casting해서 사용

    // 해결 방법(2) - compiler에게 지정하도록 함
    let my_float = 5.0;
    let my_other_float = 8.5;

    let third_float = my_float + my_other_float;

    // 해결 방법(3) - compiler의 추론 유도 (한 쪽만 type을 명시)
    let my_float: f32 = 5.0;
    let my_other_float = 8.5;

    let third_float = my_float + my_other_float;
}