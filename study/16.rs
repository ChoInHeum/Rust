/// 16. const and static
// 값을 선언하는 방법은 두 가지임
// const와 static의 차이점
// - const는 변경할 수 없는 값임, 변경할 땐 다시 선언
// - static은 const와 비슷하지만 메모리에 고정되는 것이기 때문에
// - 전역 변수로 사용할 수 있음

// const와 static은 let 키워드를 사용하지 않고
// const ~
// static ~
// 형식으로 선언함
fn main() {
    const NUMBER_OF_MONTHS: u32 = 12;
    static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
}