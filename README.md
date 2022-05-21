# Solar Car Software Team pre-commit hook

Pre-defined pre-commit hooks for the Solar Car software team.

## Installation

Ensure that you have installed pre-commit following the instructions [here](https://pre-commit.com/#install).

To use this pre-commit hook in your repo, add the following to your `.pre-commit-config.yaml` file.

Also ensure rust is installed by following instructions [here for Windows/Linux](https://www.rust-lang.org/tools/install) and [here for MacOS](https://sourabhbajaj.com/mac-setup/Rust/).

```yaml
-   repo: https://github.com/UCSolarCarTeam/Solar-Car-Pre-commit
    rev: master
    hooks:
    -   id: hook-name
```

## Hooks

### Commit message validator

```yaml
    hooks:
    -   id: solar-car-commit-msg
        stages: [commit-msg]
```
