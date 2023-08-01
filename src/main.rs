use tmux::tmux_actions;

mod command;
mod tmux;

fn main() {
    env_logger::init();

    let home_dir = std::env::var("HOME").unwrap();
    let config_folder_path = match std::env::var("XDG_CONFIG_HOME") {
        Ok(val) => val,
        _ => format!("{}/.config", home_dir),
    };

    let tmux_config_folder_path = format!("{}/tmux", config_folder_path);

    tmux_actions::select_session(tmux_config_folder_path.as_ref())
}
