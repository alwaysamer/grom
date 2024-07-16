# Grom
Grom is a CLI for note-taking. It allows you to create project-specific notes as well as quick notes, and you can also have a daily, weekly, or monthly diary.

> [!WARNING]
 This is a very early version of the project and is still under development. The code is not yet stable and may contain bugs. 
 Also my first ever Rust-project so the code is probably not the best.

## Installation
Currently, the only supported installation method is via crates.io using the following command:
```bash
cargo install grom
```
## Configuration
Grom expects a 'grom.toml' file under $HOME/.config/grom (currently only Mac and Linux are supported and tested so i don't know how Windows behaves).
The contents of the file should look like this:
```toml
# These values are the default
[core]
note_dir="~/notes" # defines where the notes are saved
editor="nvim" # which editor to use to open the file. Any command is applicable (just use the actual command not an alias)
```
## Usage
Grom provides 3 basic functionalities:
* Diarys (on a daily,weekly and monthly basis)
* Projects (Creating and opening Projects)
* Syncing via Git (Currently init,push and pull are supported)

### Diary
As mentioned above, grom provides a daily, weekly, and monthly diary. To create a diary entry, you can use the following commands:
```bash
# to create a daily diary entry. (created under -> <note_dir>/diary/<year>/<month>/<iso_week>/dd-MM-YYYY.md)
grom today

# to create a weekly diary entry (created under -> <note_dir>/diary/<year>/<month>/<iso_week>/week.md)
grom week

# to create a monthly diary entry (created under -> <note_dir>/diary/<year>/<month>/month.md)
grom month
```
If these files already exist, grom will open the existing file in the editor defined in the configuration file.
### Projects
Grom also allows you to create project-specific notes. To create/open a project, you can use the following command:
```bash
# to create a project (created under -> <note_dir>/projects/<project_name>/start.md) 
grom new <project_name>

# to open a project
grom <project_name>

# to use the interactive project selection
grom
```
### Syncing via Git
Synchronizing your notes with a git repository is also possible. To do so, you can use the following commands:
```bash
# initialiting the repository
grom init <git_url>

# pushing changes to your repo
grom push <commit_message>

# pulling changes from remote
grom pull
```
## Changelog
See [Changelog](/CHANGELOG.md)
