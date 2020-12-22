# rmake

* Simple task runner.

# Getting started

```bash
cargo install rmake
```

* Not in crates.io now.

# rmake.toml

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
