use std::env;
use std::process::Command;

fn execute_cmd(cmd: &str) -> String {
    // cmd.exe /c argument
    let temp = "/c".to_owned();
    let full_cmd = temp + cmd; // /c whoami

    let cmds: Vec<&str> = full_cmd.split(" ").collect();
    let res = Command::new("cmd.exe").args(&cmds).output().unwrap();

    let stdout = String::from_utf8_lossy(&res.stdout).to_string();
    let stderr = String::from_utf8_lossy(&res.stderr).to_string();

    if stdout.len() > 0 {
        return stdout.to_string();
    } else {
        return stderr.to_string();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let res = execute_cmd(&args[1]);
        println!("{}", res);
    } else {
        println!("[+] Usage: {} command", args[0]);
    }
}
