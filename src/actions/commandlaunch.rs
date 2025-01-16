use std::process::Command;

pub fn command_launch(exec: &str, keyword: &String) {
    let exec = exec.replace("{keyword}", &keyword);
    let commands: Vec<&str> = exec.split('&').collect();
    let num_cmds = commands.len();

    for command in commands {
        if !command.is_empty() {
            let mut parts = command.split_whitespace();
            let execute = parts.next().expect("No command found");
            let args: Vec<&str> = parts.collect();

            if num_cmds > 1 {
                Command::new(execute)
                    .args(&args)
                    .spawn()
                    .expect(&format!("Failed to execute command: {:?}", command));

            } else {
                // Synchronous execution (output)
                let output = Command::new(execute)
                    .args(&args)
                    .output()
                    .expect(&format!("Failed to execute command: {:?}", command));

                if output.status.success() {
                    println!("Command executed successfully!");
                    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
                } else {
                    eprintln!("Command failed with status: {}", output.status);
                    eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
    }
}
