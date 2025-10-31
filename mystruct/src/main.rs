struct Rect {
  width:u32,
  height:u32
}

fn get_area(rectref:&Rect)->u32 {
  //C言語っぽく書くとこう
  //return (*rectref).width * (*rectref).height;
  return rectref.width * rectref.height;
}

fn main() {
  let rect1 = Rect{width:2, height:3};
  let area1 = rect1.width * rect1.height;
  let rect2 = Rect{width:4, height:6};
  let area2 = get_area(&rect2);
  println!("area1 = {}", area1);
  println!("area2 = {}", area2);
}
