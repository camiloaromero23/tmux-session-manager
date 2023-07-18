mod command;

use command::{
    command_ran_successfully, get_attach_to_window_command, get_command_output, 
    parse_window_command, run_command,
};

use rust_fzf::fzf_select;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowConfig {
    window_name: Option<String>,
    command: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SessionConfig {
    session_dir: String,
    windows: Vec<WindowConfig>,
}

fn main() {
    env_logger::init();

    let fzf_input = get_command_output(
        r"ls $HOME/.config/tmux/session*.json | sed 's/.*\///; s/\.json//' | sort -r",
    );

    let config_file_name = fzf_select(fzf_input) + ".json";

    let home_dir = std::env::var("HOME").unwrap();
    let config_folder_path = match std::env::var("XDG_CONFIG_HOME") {
        Ok(val) => val,
        _ => "~/.config".replace("~", home_dir.as_str()),
    };

    let tmux_config_folder_path = format!("{}/tmux", config_folder_path);

    let session_config_path = format!("{}/{}", tmux_config_folder_path, config_file_name);

    let file = std::fs::read_to_string(session_config_path).expect("Unable to read file");

    let session: SessionConfig = serde_json::from_str(&file)
        .expect("Session does not have correct format. Watch file structure in base_session.json");

    let session_name = session
        .session_dir
        .split("/")
        .last()
        .expect("No session name provided");

    let session_exists = command_ran_successfully(format!("tmux has-session -t {}", session_name));

    let is_running_inside_tmux = std::env::var("TMUX").is_ok();

    let attach_to_window_command =
        get_attach_to_window_command(session_name, is_running_inside_tmux);

    if session_exists {
        log::debug!("Session {} already exists", session_name);
        run_command(attach_to_window_command);
        return;
    }

    let first_window = session.windows.get(0).expect("No windows provided");

    let window_name = match first_window.window_name.to_owned() {
        Some(name) => format!(" -n {}", name),
        _ => "".to_owned(),
    };

    let tmux_session_command = parse_window_command(first_window.command.to_owned(), Some(true));
    let tmux_session_command = format!(
        "tmux new-session -d -c {} -s {}{}{}",
        session.session_dir, session_name, window_name, tmux_session_command
    );

    let mut tmux_commands = vec![tmux_session_command];

    session.windows.into_iter().skip(1).for_each(|window| {
        let window_name = match window.window_name {
            Some(name) => format!(" -n {}", name),
            _ => "".to_owned(),
        };
        let command = parse_window_command(window.command, None);
        let command = format!(
            "tmux new-window -c {}{} -t {}{}",
            session.session_dir, window_name, session_name, command
        );

        tmux_commands.push(command);
    });

    tmux_commands.push(attach_to_window_command);

    tmux_commands.into_iter().for_each(|command| {
        log::debug!("Executing command: {}", command);

        run_command(command);
    });
}
