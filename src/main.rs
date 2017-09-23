#[cfg(target_os = "linux")]
extern crate users;

#[cfg(target_os = "linux")]
use users::{UsersCache, Users};

#[cfg(target_os = "linux")]
fn main() {
    let cache = UsersCache::new();
    let current_uid = cache.get_current_uid();
    if current_uid == 0 {
        println!("Successfully detected root permissions")
    } else {
        println!("Please run tunneldigger as Root!")
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Tunneldigger only supports Linux.");
}