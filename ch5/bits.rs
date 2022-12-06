fn main() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    println!("bit representation: {n_bits}");
   //let sign_bits = n_bits >> 31;
    //println!("compare after shift: {n_bits} | {sign_bits}");

    let exponent_ = n_bits >> 23;
    let exponent_ = exponent_ & 0xff;
    let exponent_ = (exponent_ as u32) - 127;

}