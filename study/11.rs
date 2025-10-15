/// 11. Display and debug
/// 어떤 변수는 단순히 println!을 사용하여 출력할 수 있지만
/// () 과 같이 아무것도 없는(void)는 출력할 수 없음.
/// 그래서 (void)과 같이 println!으로 출력할 수 없을 것을 출력하기 위해 debug print를 사용
/// debug print에는 {:?} 과 {:#?} <- called "pretty printing" 다른 형식으로 더 적어줌?
fn main() {
    let doesnt_print = ();
    println!("This will not print: {}", doesnt_print); // Error
}

// 출력 후 줄바꿈이 필요 없으면 print!를 사용해도 됨
fn main() {
    print!("Tihs will not print a new line");
    println!(" so this wile be on the same line");
}
// 출력: This will not print a new line so this will be on the same line

/// 가장 작은 수와 가장 큰 수
fn main() {
    println!("The smallest i8 is {} and the biggest i8 is {}.", i8::MIN, i8::MAX);
    println!("The smallest u8 is {} and the biggest u8 is {}.", u8::MIN, u8::MAX);
    println!("The smallest i16 is {} and the biggest i16 is {}.", i16::MIN, i16::MAX);
    println!("The smallest u16 is {} and the biggest u16 is {}.", u16::MIN, u16::MAX);
    println!("The smallest i32 is {} and the biggest i32 is {}.", i32::MIN, i32::MAX);
    println!("The smallest u32 is {} and the biggest u32 is {}.", u32::MIN, u32::MAX);
    println!("The smallest i64 is {} and the biggest i64 is {}.", i64::MIN, i64::MAX);
    println!("The smallest u64 is {} and the biggest u64 is {}.", u64::MIN, u64::MAX);
    println!("The smallest i128 is {} and the biggest i128 is {}.", i128::MIN, i128::MAX);
    println!("The smallest u128 is {} and the biggest u128 is {}.", u128::MIN, u128::MAX);
}
// 출력:
// The smallest i8 is -128 and the biggest i8 is 127.
// The smallest u8 is 0 and the biggest u8 is 255.
// The smallest i16 is -32768 and the biggest i16 is 32767.
// The smallest u16 is 0 and the biggest u16 is 65535.
// The smallest i32 is -2147483648 and the biggest i32 is 2147483647.
// The smallest u32 is 0 and the biggest u32 is 4294967295.
// The smallest i64 is -9223372036854775808 and the biggest i64 is 9223372036854775807.
// The smallest u64 is 0 and the biggest u64 is 18446744073709551615.
// The smallest i128 is -170141183460469231731687303715884105728 and the biggest i128 is 170141183460469231731687303715884105727.
// The smallest u128 is 0 and the biggest u128 is 340282366920938463463374607431768211455.
