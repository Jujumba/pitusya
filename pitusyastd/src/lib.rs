#[no_mangle]
pub extern "C" fn print(n: f64) -> f64 {
    println!("{n:.2}");
    n
}
pub extern "C" fn putchar(c: f64) -> f64 {
    print!("{}", c as u8 as char);
    c
}
#[no_mangle]
pub extern "C" fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}
#[no_mangle]
pub extern "C" fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}