/// 14. More about printing
/// "" 안에서 \n => 줄바꿈 \t => 탭
fn main() {
    print!("\t Start with a tab\nand move to new line");
}
/// 출력
///     Start with a tab
/// and move to a new line

// "" 안에서는 얼마든지 줄바꿈을 할 수 있다
fn main() {
    println!("Inside quotes
you can write over
many lines
and it will print just fine.");

    println!("If yout forget to write
    on the left sid, the spaces
    will be added when you print.");
}
/// <출력>
/// Inside quotes
/// yout can write over
/// many lines
/// and it will print just fine.
/// If yout forget to write
///     on the left side. the spaces
///     will be added when you print.

// \n와 \t를 문자열로 출력하는 방법
fn main() {
    println!("Here are two escape characters: \\n and \\t");
}
/// <출력>
/// Here are two escape characters: \n and \t

// 가끔 "" 안에 많은 \, " 등을 사용할 때, r#"~"#을 사용하면 완전히 출력할 수 있다.
fn main() {
    println!("He said, \"you can find th file at c:\\files\\my_documents\\file.txt.\" Then I found file.");
    println!(r#"He said, "you can find the file at c:\files\my_documents\file.txt." Then I found the file."#);
}
/// <출력>
/// He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file.
/// He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file.

// "" 안에 #을 넣고 싶으면 r##"~"##을 사용하면 #을 포함하여 "" 안의 모든 내용을 출력할 수 있음
fn main() {
    let my_string = "'Ice to see you,' he said.";
    let quote_string = r#""Ice to see you," he said."#;
    let hashtag_string = r##"The hsahtag #IceToSeeYou had become ver popular."##;
    let many_hashtags = r####"You don't have to type ### to use a hastag. you can just use #."####;

    println!("{}\n{}\n{}\n{}\n", my_string, quote_string, hashtag_string, many_hashtags);
}
/// <출력>
// 'Ice to see you,' he said.
// "Ice to see you," he said.
// The hsahtag #IceToSeeYou had become ver popular.
// You don't have to type ### to use a hastag. you can just use #.

/// r#은 let과 같은 rust에서 지정된 키워드를 변수 이름으로 지정할 수 있게 해준다.
fn main() {
    let r#let = 6;
    let mut r#mut = 10;
}
// <ex>
fn r#return() -> u8 {
    println!("Here si your number");
    8
}

fn main() {
    let my_number = r#return();
    println!("{}", my_number);
}
// <출력>
// Here is your number
// 8

// 뒤 내용은 생략함