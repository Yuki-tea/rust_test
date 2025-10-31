fn main() {
  let ok = String::from("Okです");
  let third = ok.chars().nth(2);
  if third != None{
    println!("「{}」の3番目の文字は「{}」", ok, third.unwrap());
  }

  let sixth = ok.chars().nth(5);
  if sixth == None{
    println!("「{}」の6番目の文字はありません", ok);
  }

  if let Some(val) = &Some("中の値") {
    println!("{}をとりだしました", *val);
  }

  let first = ok.chars().nth(0);
  if let Some(ch) = &first{
    println!("「{}」の最初の文字は「{}」", ok, *ch);
  }

  if Some('O') == first{
    println!("{}", "最初の文字は分かっていた");
  }
}
