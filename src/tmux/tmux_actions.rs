use crate::tmux::sessions::SessionConfig;
use itertools::Itertools;
use rust_fzf;

use crate::command::run_command;

use super::tmux;

pub fn select_session(tmux_config_folder_path: &str) {
    let mut configured_sessions = tmux::get_configured_sessions(tmux_config_folder_path);

    let running_tmux_sessions = tmux::list_sessions();

    configured_sessions.extend(running_tmux_sessions);
    let configured_sessions = configured_sessions
        .into_iter()
        .unique()
        .sorted_unstable()
        .collect();

    let selected_session = rust_fzf::select(configured_sessions, vec![]);

    if selected_session.is_empty() {
        return;
    }

    let session_exists = tmux::session_exists(selected_session.as_ref());

    let attach_to_window_command = tmux::get_attach_to_window_command(selected_session.as_ref());

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

    let attach_to_window_command = tmux::get_attach_to_window_command(selected_session.as_ref());

    let tmux_session_command =
        tmux::create_session_command(&session_config, selected_session.as_ref());

    let mut tmux_commands = vec![tmux_session_command];

    let tmux_window_commands: Vec<String> = session_config
        .windows
        .clone()
        .into_iter()
        .skip(1)
        .map(|window| {
            tmux::create_window_command((&session_config, &window, selected_session.as_ref()))
        })
        .collect();

    tmux_commands.extend(tmux_window_commands);

    tmux_commands.push(attach_to_window_command);

    tmux_commands.into_iter().for_each(|command| {
        run_command(command);
    });
}

pub fn kill_session() {
    let active_sessions = tmux::list_sessions();
    let session_to_kill = rust_fzf::select(active_sessions, vec![]);

    if session_to_kill.is_empty() {
        return;
    }

    let command = tmux::kill_session(session_to_kill.as_ref());

    run_command(command);
}
