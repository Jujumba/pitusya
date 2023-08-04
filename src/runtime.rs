#[no_mangle]
pub extern "C" fn print(n: f64) -> f64 {
    println!("{n:.2}");
    0f64
}