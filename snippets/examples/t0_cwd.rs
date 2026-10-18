// region: cwd
use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    fs::create_dir_all("orders")?;
    fs::write("orders/today.txt", "BRG-6204,40\n")?;
    let today = Path::new("orders/today.txt");
    println!("from here:   {}", today.exists());
    env::set_current_dir("orders")?;
    println!("from orders: {}", today.exists());
    Ok(())
}
// endregion
