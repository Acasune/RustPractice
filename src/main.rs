fn main() {
  let mut cnt = 0;

  let result = loop{
      println!("count: {}", cnt);
      cnt += 1;
      if cnt == 10{
          break cnt;
      }
  };
}
