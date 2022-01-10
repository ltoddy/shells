# shells

execute in the shell

```toml
[dependencies.shells]
git = "https://github.com/ltoddy/shells"
branch = "master"
```

#### Usage

```rust
use std::io;

use shells::bash;

pub fn main() -> io::Result<()> {
    let (code, stdout, stderr) = bash!("echo '{} + {}' | bc", 1, 2);
    assert_eq!(code, 0);
    assert_eq!(stdout, String::from("3\n"));
    assert_eq!(stderr, String::new());
    Ok(())
}
```
