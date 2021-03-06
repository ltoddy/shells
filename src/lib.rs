// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Execute in the shell
//! ```toml
//! [dependencies.shells]
//! git = "https://github.com/ltoddy/shells"
//! branch = "master"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use std::io;
//!
//! use shells::bash;
//!
//! pub fn main() -> io::Result<()> {
//!     let (code, stdout, stderr) = bash!("echo '{} + {}' | bc", 1, 2)?;
//!     assert_eq!(code, 0);
//!     assert_eq!(stdout, String::from("3\n"));
//!     assert_eq!(stderr, String::new());
//!     Ok(())
//! }
//! ```

use std::io;
use std::process::Command;

#[macro_export]
macro_rules! sh {
    ($($arg:tt)*) => { $crate::execute_by("sh", &format!($($arg)*)) };
}

#[macro_export]
macro_rules! bash {
    ($($arg:tt)*) => { $crate::execute_by("bash", &format!($($arg)*)) };
}

#[macro_export]
macro_rules! zsh {
    ($($arg:tt)*) => { $crate::execute_by("zsh", &format!($($arg)*)) };
}

#[doc(hidden)]
pub fn execute_by(shell: &str, cmd: &str) -> io::Result<(i32, String, String)> {
    let mut command = Command::new(shell);
    command.arg("-c").arg(cmd);

    command.output().map(|output| {
        let code = output.status.code().unwrap_or_else(|| if output.status.success() { 0 } else { -1 });
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        (code, stdout, stderr)
    })
}

#[cfg(test)]
pub mod test {
    #[test]
    pub fn test_sh() {
        let result = sh!("echo '{} + {}' | bc", 1, 2);
        assert!(result.is_ok());

        let (code, stdout, stderr) = result.unwrap();
        assert_eq!(code, 0);
        assert_eq!(stdout, String::from("3\n"));
        assert_eq!(stderr, String::new());
    }

    #[test]
    pub fn test_bash() {
        let result = bash!("echo '{} + {}' | bc", 1, 2);
        assert!(result.is_ok());

        let (code, stdout, stderr) = result.unwrap();
        assert_eq!(code, 0);
        assert_eq!(stdout, String::from("3\n"));
        assert_eq!(stderr, String::new());
    }

    #[test]
    pub fn test_zsh() {
        let result = zsh!("echo '{} + {}' | bc", 1, 2);
        assert!(result.is_ok());

        let (code, stdout, stderr) = result.unwrap();
        assert_eq!(code, 0);
        assert_eq!(stdout, String::from("3\n"));
        assert_eq!(stderr, String::new());
    }
}
