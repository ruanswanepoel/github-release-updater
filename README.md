# github-release-updater
Keeps a program up to date with the latest release on GitHub.

## Usage
Currently this is a very simple script that runs infinately on the main thread. The reccomended way to run this program is to use a process manager like [pm2](https://pm2.keymetrics.io/). For example:
```bash
pm2 start github-release-updater.exe
```
