# About

* Simple task runner as `make`.

# Getting started

```bash
cargo install rmake
```

# rmake.toml is as

```
[env]
MY_ENV_01 = "administrator@example.com"
MY_ENV_02 = "2147483647"
MY_ENV_03 = "true"

[[tasks]]
description = "Hello, rmake!"
name = "default task"
depends_on = []
command = [
	["echo", "%MY_ENV_02%"]
]
```
