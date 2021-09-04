[![Rust Application Build Workflow](https://github.com/mass10/rmake/actions/workflows/build.yml/badge.svg)](https://github.com/mass10/rmake/actions/workflows/build.yml)

# About

* A simple task runner like `make`.

# Getting started

```bash
cargo install rmake
```

# How to run the first task

```bash
rmake
```

# How to run the specified task

```bash
rmake {name of task}
```

# Show usage

```bash
rmake --help
```

# rmake.toml is as

```toml
[env]
MY_ENV_01 = "Hello,"

[variables]
MY_VAR_02 = "World!"

[[tasks]]
description = "anything"
name = "world"
depends_on = ["hello"]
command = [
	"ECHO {{MY_VAR_02}}",
]

[[tasks]]
description = "anything"
name = "hello"
depends_on = []
command = [
	"!MKDIR .tmp", # SAFE with "!"
	"!DEL /S /Q .tmp\\*", # SAFE with "!"
	"ECHO %MY_ENV_01%",
]
```

# Future Plans
* Build causes NO AFFECTS when no modified files found.
