# Solar Car Software Team pre-commit hook

Pre-defined pre-commit hooks for the Solar Car software team.

## Installation

Ensure that you have installed pre-commit following the instructions [here](https://pre-commit.com/#install).

To use this pre-commit hook in your repo, add the following to your `.pre-commit-config.yaml` file.

```yaml
repos:
-   repo: #TO-DO: update this url
    rev: master
    hooks:
    -   id: hook-name
```

## Hooks

### Commit message validator

```yaml
    hooks:
    -   id: solar-car-commit-msg
```
