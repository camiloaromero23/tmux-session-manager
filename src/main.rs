mod command;

use command::{
    command_ran_successfully, get_attach_to_window_command, get_command_output,
    parse_window_command, run_command,
};

use itertools::Itertools;
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

    let home_dir = std::env::var("HOME").unwrap();
    let config_folder_path = match std::env::var("XDG_CONFIG_HOME") {
        Ok(val) => val,
        _ => format!("{}/.config", home_dir),
    };

    let tmux_config_folder_path = format!("{}/tmux", config_folder_path);

    let sed_replace_regex = r"s/.*\///; s/\.json//";

    let mut configured_sessions = get_command_output(
        format!(
            r"ls {}/sessions/*.json | sed '{}'",
            tmux_config_folder_path, sed_replace_regex,
        )
        .as_ref(),
    );

    let running_tmux_sessions = get_command_output("tmux ls -F '#{session_name}'");

    configured_sessions.extend(running_tmux_sessions);
    let configured_sessions = configured_sessions
        .into_iter()
        .unique()
        .sorted_unstable()
        .collect();

    let selected_session = fzf_select(configured_sessions);

    let session_exists =
        command_ran_successfully(format!("tmux has-session -t {}", selected_session));

    let attach_to_window_command = get_attach_to_window_command(selected_session.as_ref());

    if session_exists {
        log::debug!("Session {} already exists", selected_session);
        run_command(attach_to_window_command);
        return;
    }

    let config_file_name = format!("{}.json", selected_session);

    let session_config_path = format!("{}/sessions/{}", tmux_config_folder_path, config_file_name);

    let selected_session_file =
        std::fs::read_to_string(session_config_path).expect("Unable to read file");

    let session_config: SessionConfig = serde_json::from_str(&selected_session_file)
        .expect("Session does not have correct format. Watch file structure in base_session.json");

    let attach_to_window_command = get_attach_to_window_command(selected_session.as_ref());

    let first_window = session_config.windows.get(0).expect("No windows provided");

    let window_name = match first_window.window_name.to_owned() {
        Some(name) => format!(" -n {}", name),
        _ => "".to_owned(),
    };

    let tmux_session_command = parse_window_command(first_window.command.to_owned(), Some(true));
    let tmux_session_command = format!(
        "tmux new-session -d -c {} -s {}{}{}",
        session_config.session_dir, selected_session, window_name, tmux_session_command
    );

    let mut tmux_commands = vec![tmux_session_command];

    session_config
        .windows
        .into_iter()
        .skip(1)
        .for_each(|window| {
            let window_name = match window.window_name {
                Some(name) => format!(" -n {}", name),
                _ => "".to_owned(),
            };
            let command = parse_window_command(window.command, None);
            let command = format!(
                "tmux new-window -c {}{} -t {}{}",
                session_config.session_dir, window_name, selected_session, command
            );

            tmux_commands.push(command);
        });

    tmux_commands.push(attach_to_window_command);

    tmux_commands.into_iter().for_each(|command| {
        run_command(command);
    });
}
