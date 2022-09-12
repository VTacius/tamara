use std::process::Command;

fn main() {
    let salida = Command::new("sh").args(["-c"]).arg("sudo setcap cap_net_raw+ep target/debug/tamara").output().unwrap();
    println!("{:?}", salida.stdout);
}