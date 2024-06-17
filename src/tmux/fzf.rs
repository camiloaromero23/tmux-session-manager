use rust_fzf;

pub fn select(options: Vec<String>, header_title: &str) -> Option<String> {
    let selected_session = rust_fzf::select(
        options,
        vec![
            "--height=50%".to_string(),
            "--tmux".to_string(),
            "--border=rounded".to_string(),
            format!("--border-label={}", header_title),
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
