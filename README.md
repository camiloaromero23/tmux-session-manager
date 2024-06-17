# Tmux Session Manager

To install `tmux-session-manager`, run the following command:

```bash
cargo install tmux-session-manager
```

To use the `tmux-session-manager` first make sure to have installed the following dependencies on your system:

## Dependencies
- [fzf](https://github.com/junegunn/fzf)
- [tmux](https://github.com/tmux/tmux)

The available commands are:

## Select a session (this includes both active and pre-configured sessions)

```bash
tmux-session-manager
```

## Select an active session

```bash
tmux-session-manager -s
tmux-session-manager --select
```

## Kill an active session

```bash
tmux-session-manager -k
tmux-session-manager --kill
```

## Usage tips
You can add keybinds to available commands both on zsh or tmux for making your experience blazingly fast 🚀🤘🏽
