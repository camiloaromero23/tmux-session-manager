use clap::Parser;
use tmux::tmux_actions;

mod command;
mod tmux;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    kill_session: bool,
    #[arg(short, long)]
    select_active_session: bool,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let home_dir = std::env::var("HOME").unwrap();
    let config_folder_path = match std::env::var("XDG_CONFIG_HOME") {
        Ok(val) => val,
        _ => format!("{home_dir}/.config"),
    };

    let tmux_config_folder_path = format!("{config_folder_path}/tmux");

    if args.kill_session {
        return tmux_actions::kill_session();
    }

    if args.select_active_session {
        return tmux_actions::select_active_session();
    }

    tmux_actions::select_session(&tmux_config_folder_path)
}
