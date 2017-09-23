#[cfg(target_os = "linux")]
extern crate users;

#[cfg(target_os = "linux")]
use users::{UsersCache};

#[cfg(target_os = "linux")]
fn main() {
    let cache = UsersCache::new();
    let current_uid = cache.get_current_uid();
    println!("Your UID is {}", current_uid);
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Tunneldigger only supports Linux.");
}