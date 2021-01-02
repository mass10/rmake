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

```
[env]
MY_ENV_01 = "Hello,"
MY_ENV_02 = "World!"

[variables]
MY_VAR_01 = "01"
MY_VAR_02 = "02"

[[tasks]]
description = "anything"
name = "default"
depends_on = ["common"]
command = [
	"ECHO %MY_ENV_02%",
	"ECHO {{MY_VAR_02}}",
]

[[tasks]]
description = "anything"
name = "common"
depends_on = []
command = [
	"ECHO %MY_ENV_01%",
	"ECHO {{MY_VAR_01}}",
]
```
