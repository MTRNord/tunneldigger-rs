fn main() {
    if cfg!(target_os = "linux") {
        main_a()
    } else {
        main_a()
    }
}

#[cfg(target_os = "linux")]
fn main_a() {

}

#[cfg(not(target_os = "linux"))]
fn main_a() {
    println!("Tunneldigger only supports Linux.");
}