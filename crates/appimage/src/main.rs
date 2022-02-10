mod list;

fn main() {
    let mut s = String::new();
    println!("{}", s);
    let packages = list::list_installed();
    if let Some(packages) = packages {
        println!("{:?}", packages);
    } else {
        println!("No packages")
    }
    s = "1".to_string();
    println!("{}", s);

    match s.as_ref() {
        "hello" => {}
        _ => {}
    }
}
