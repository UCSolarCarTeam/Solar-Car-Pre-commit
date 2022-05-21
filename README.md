# Solar Car Software Team pre-commit hook

Pre-defined pre-commit hooks for the Solar Car software team.

## Installation

Ensure that you have installed pre-commit following the instructions [here](https://pre-commit.com/#install).

To use this pre-commit hook in your repo, add the following to your `.pre-commit-config.yaml` file.

Also ensure rust is installed by following instructions [here for Windows/Linux](https://www.rust-lang.org/tools/install) and [here for MacOS](https://sourabhbajaj.com/mac-setup/Rust/).

```yaml
-   repo: https://github.com/UCSolarCarTeam/Solar-Car-Pre-commit
    rev: # tag e.g. v.0.1.0
    hooks:
    -   id: hook-name
```

## Hooks

### Commit message validator

To use this commit hook, add the following to your `.pre-commit-config.yaml`.
```yaml
    hooks:
    -   id: solar-car-commit-msg
        stages: [commit-msg]
```

This pre-commit hooks verifies that the commit message follows the solar car commit message guideline.
Commit messages must begin with an SFT-XXX tag and be followed by a short description of the commit.
Commit messages must be between 20 and 72 characters.

| Example commit message                                                  | Hook result |
| ----------------------------------------------------------------------- | ----------- |
| SFT-1 Commit message                                                    | ✓ Pass     |
| Commit message                                                          | ✕ Fail     |
| This commit message is waaaaaayyyyyy tooooooooooooooooo longggggggggggg | ✕ Fail            |