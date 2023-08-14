# Contributing

## Bug report

For now [GitHub Bug Traker](https://github.com/zefr0x/ianny/issues) is used for this project.

## Development

- The `stable` [Rust](https://www.rust-lang.org/) programming language is used mainly in this project.
- Dependencies managment and building is handled using [Cargo](https://doc.rust-lang.org/stable/cargo/).

### Style

- You should document every thing to keep the code easy for reading. Every file, every function and any line that need a comment.

#### Setup `pre-commit`

To make every thing easy [**`pre-commit`**](https://pre-commit.com/) is used in this project, it should run in every commit, so you shouldn't commit any thing without checking it.

First install it:

```shell
pip install pre-commit
```

> It's better to use you OS's package manager to install it.

Then add it as a git hook while you are inside the repository:

```shell
pre-commit install
```

<!-- TODO:  -->
<!-- # Translation -->
