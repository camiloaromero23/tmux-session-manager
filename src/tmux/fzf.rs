use rust_fzf;

pub fn select(
    options: Vec<String>,
    header_title: &str,
    active_session_preview: bool,
) -> Option<String> {
    let preview_command = if active_session_preview {
        "tmux display-message -p -F '#{window_index}' | tmux capture-pane -ep -t {}:".to_string()
    } else {
        "".to_string()
    };

    let selected_session = rust_fzf::select(
        options,
        vec![
            "--tmux=center,85%".to_string(),
            "--border=rounded".to_string(),
            "--preview-window=right:65%".to_string(),
            format!("--preview={preview_command}"),
            format!("--border-label={header_title}"),
        ],
    );

    if selected_session.is_err() {
        return None;
    }

    let selected_session = selected_session.unwrap();

    if selected_session.is_empty() {
        return None;
    }

    let selected_session = selected_session.get(0).unwrap();

    return Some(selected_session.to_string());
}
