const READY_AMOUNT: i32 = 2;
fn main() {
    let test = 8;
    let (mut missiles, ready): (i32,i32) = (READY_AMOUNT*4, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
