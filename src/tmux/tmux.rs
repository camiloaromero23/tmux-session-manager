use crate::command::{command_ran_successfully, get_command_output};

use super::sessions::{SessionConfig, WindowConfig};

pub struct CreateWindowCommandParams<'a> {
    pub session: &'a SessionConfig,
    pub window: WindowConfig,
    pub selected_session: &'a str,
    pub session_index: usize,
}

pub fn list_sessions() -> Vec<String> {
    return get_command_output("tmux ls -F '#{session_name}'");
}

pub fn session_exists(selected_session: &str) -> bool {
    return command_ran_successfully(format!(
        "tmux ls -F '#{selected_session}' | grep -x {selected_session}"
    ));
}

pub fn create_session_command(session_config: &SessionConfig, selected_session: &str) -> String {
    let first_window = session_config
        .windows
        .get(0)
        .expect("No windows provided")
        .to_owned();

    let window_name = match first_window.window_name.to_owned() {
        Some(name) => format!(" -n \"{name}\""),
        _ => "".to_owned(),
    };

    let window_command = window_command(selected_session, first_window, 1);
    let tmux_session_command = format!(
        "tmux new-session -d -c {} -s {selected_session}{window_name}; {window_command}",
        session_config.session_dir
    );
    return tmux_session_command;
}

pub fn create_window_command(create_window_command_params: CreateWindowCommandParams) -> String {
    let CreateWindowCommandParams {
        session,
        window,
        selected_session,
        session_index,
    } = create_window_command_params;

    let window_name = match window.window_name.to_owned() {
        Some(name) => format!(" -n \"{name}\""),
        _ => "".to_owned(),
    };
    let command = format!(
        "tmux new-window -c {}{window_name} -t {selected_session}:{session_index}",
        session.session_dir
    );
    let window_command = window_command(selected_session, window, session_index);
    let command = format!("{command}; {window_command}");
    return command;
}

fn window_command(session_name: &str, window: WindowConfig, session_index: usize) -> String {
    if window.command.is_empty() {
        return "".to_owned();
    }

    let command = format!(
        "tmux send-keys -t {session_name}:{session_index} \"{}\" Enter",
        window.command
    );

    return command;
}

pub fn get_configured_sessions(tmux_config_folder_path: &str) -> Vec<String> {
    let sed_replace_regex = r"s/.*\///; s/\.json//";

    return get_command_output(&format!(
        r"ls {tmux_config_folder_path}/sessions/*.json | sed '{sed_replace_regex}'",
    ));
}

pub fn get_attach_to_window_command(session_name: &str) -> (String, bool) {
    let session_exists = session_exists(session_name);
    let is_running_inside_tmux = std::env::var("TMUX").is_ok();

    if !is_running_inside_tmux {
        let cmd = format!("tmux attach-session -t {session_name}:1");
        return (cmd, session_exists);
    }

    if session_exists {
        let cmd = format!("tmux switch-client -t {session_name}");
        return (cmd, session_exists);
    }

    let cmd = format!("tmux switch-client -t {session_name}:1");

    return (cmd, session_exists);
}

pub fn get_kill_session_command(session_name: &str) -> String {
    return format!("tmux kill-session -t {session_name}");
}
