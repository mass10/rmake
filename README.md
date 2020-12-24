# About

* Simple task runner like `make`.

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
MY_ENV_01 = "administrator@example.com"
MY_ENV_02 = "2147483647"

[[tasks]]
description = "Hello, rmake!"
name = "default"
depends_on = ["common"]
command = [
	["echo", "%MY_ENV_01%"],
	["echo", "$MY_ENV_02"]
]

[[tasks]]
description = "My Common Library"
name = "common"
depends_on = []
command = [
	["echo", "%MY_ENV_01%"],
	["echo", "$MY_ENV_02"]
]
```
