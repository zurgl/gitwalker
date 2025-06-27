# GitWalker

## Description

Small project which allow to retrieve remote origin from a
folder containing lot of github repositories.

## Usage

Walk-git --folder `folder-name` or default to `github`.

```bash
gitwalker --folder my_github_root_dir | tr -d '[]," ' > github_remote.txt
```

Its going to retrieve all the remote origin put into a `github_remote.txt` file.
Then, it will erase all the directory, freeing space.
