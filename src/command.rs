pub fn parse_window_command(command: String, is_session: Option<bool>) -> String {
    if command == "" {
        return command;
    }

    if is_session == Some(true) {
        return format!(" \"{}; zsh\"", command);
    }

    return format!(": \"{}; zsh\"", command);
}

pub fn run_command(command: String) -> std::process::ExitStatus {
    return std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait for command");
}

pub fn command_ran_successfully(command: String) -> bool {
    return run_command(command).success();
}

pub fn get_attach_to_window_command(session_name: &str, is_running_inside_tmux: bool) -> String {
    if is_running_inside_tmux {
        return format!("tmux switch-client -t {}", session_name);
    }

    return format!("tmux attach-session -t {}", session_name);
}

pub fn get_command_output(command: &str) -> Vec<String> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command")
        .wait_with_output()
        .expect("Failed to wait for command");

    return String::from_utf8(output.stdout)
        .expect("Failed to parse output")
        .lines()
        .map(|line| line.to_owned())
        .collect();
}
