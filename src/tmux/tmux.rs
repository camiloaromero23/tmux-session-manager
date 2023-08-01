use crate::command::{command_ran_successfully, get_command_output, parse_window_command};

use super::sessions::{SessionConfig, WindowConfig};

pub fn list_sessions() -> Vec<String> {
    return get_command_output("tmux ls -F '#{session_name}'");
}

pub fn session_exists(selected_session: &str) -> bool {
    return command_ran_successfully(format!("tmux has-session -t {}", selected_session));
}

pub fn create_session_command(session_config: &SessionConfig, selected_session: &str) -> String {
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
    return tmux_session_command;
}

pub fn create_window_command(
    (session_config, window, selected_session): (&SessionConfig, &WindowConfig, &str),
) -> String {
    let window_name = match window.window_name.to_owned() {
        Some(name) => format!(" -n {}", name),
        _ => "".to_owned(),
    };
    let command = parse_window_command(window.command.to_owned(), None);
    let command = format!(
        "tmux new-window -c {}{} -t {}{}",
        session_config.session_dir, window_name, selected_session, command
    );
    return command;
}

pub fn get_configured_sessions(tmux_config_folder_path: &str) -> Vec<String> {
    let sed_replace_regex = r"s/.*\///; s/\.json//";

    return get_command_output(
        format!(
            r"ls {}/sessions/*.json | sed '{}'",
            tmux_config_folder_path, sed_replace_regex,
        )
        .as_ref(),
    );
}

pub fn get_attach_to_window_command(session_name: &str) -> String {
    let is_running_inside_tmux = std::env::var("TMUX").is_ok();

    if is_running_inside_tmux {
        return format!("tmux switch-client -t {}", session_name);
    }

    return format!("tmux attach-session -t {}:1", session_name);
}
