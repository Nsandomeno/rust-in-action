use std::collections::BTreeMap;

fn main() {
  let mut voc = BTreeMap::new();

  voc.insert(3_697_098, "Amsterdam");
  voc.insert(1_300_000, "Middelburg");

  for (guilders, kamer) in &voc {
    println!("{} invested {}", kamer, guilders);
  }
  println!("smaller chambers: ");
  for (_guilders, kamer) in voc.range(0..2_000_000) {
    print!("{} ", kamer);
  }
  println!("");
}
