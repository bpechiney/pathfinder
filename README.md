# Pathfinder

## Bootstrapping

You will need to install [rustup](https://rustup.rs/) and [just](https://github.com/casey/just).
Depending on your operating system, you will need to run one of two scripts.

### Mac

1. install brew by running `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
2. run `chmod +x ./bootstrap.sh`
3. run `./bootstrap.sh`

### Windows

1. get the fully-qualified path to `Bootstrap.ps1`
2. run `powershell -Command "Start-Process PowerShell -Verb RunAs -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File """<path>"""'"`
