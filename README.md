# Tauri + React

This template should help get you started developing with Tauri and React in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## CLI

```
connect <addr>
create <name> [ -p <password> ] [ -f ]
add <id> [ -p <password> ] [ -f ]
list
copy [ -i <id> | -n <name> ] [ --echo ]
paste [ -i <id> | -n <name> ] [ -d <data_type> ] [ <content> | -f <file> ]
remove [ <id> | <name> ]
delete <id> [ -p <password> ]
```

## Config File

```toml
server="localhost:8383"

[[clipboards]]
id=100004
name="test"
password="bXlwYXNzCg==" # base64 encoded for "mypass"

[[clipboards]]
id=100005
name="another test"
```

## TODO
- Systray
- Names are independent to each device:
  - [ ] Add optional `name` field to `AddArgs` (Seperate `AddDeleteArgs`)
  - [ ] Ask for name in cli while `add`
  - [ ] Add name field in GUI for `Add Clipboard`
  - [ ] Store the user's input locally