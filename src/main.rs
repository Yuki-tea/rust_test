struct Book {
  title: String,
  pages:u32
}

fn main() {
  //データ型は&str
  //渡しているのは文字列を格納するメモリ上の位置の参照
  let rust_str = "赤さび";
  let true_string = String::from("これは文字列です");

  let book1 = Book{
    // 以下は間違い文字列リテラルは&str型
    // title: "楽しいプログラミング",
    title:String::from("楽しいプログラミング"),
    pages:210
  };

  let ok = String::from("Okです");

  println!("Rustの意味は「{}」らしいです", rust_str);
  println!("{}", true_string);
  println!("書籍『{}』は全{}ページ", book1.title, book1.pages);

  //バイト単位で数えてしまう(英数字1バイト、日本語3バイト)
  println!("「{}」の文字数は{}?", ok, ok.len());
  println!("いいえ、{}です", ok.chars().count());
}
