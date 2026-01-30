use loco_rs::hash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = "Test123456";
    let hash = hash::hash_password(password)?;
    println!("{}", hash);
    Ok(())
}
