# Rustastic Password

[![Build Status](https://travis-ci.org/conradkdotcom/rpassword.svg?branch=master)](https://travis-ci.org/conradkdotcom/rpassword)
[![Build status](https://ci.appveyor.com/api/projects/status/h7ak407y28k0ufw2?svg=true)](https://ci.appveyor.com/project/conradkdotcom/rpassword)

This [Rust](http://www.rust-lang.org/) package allows you to safely read
passwords from standard input in a console application.

You can build the documentation with `cargo doc` or [view it online](https://docs.rs/rpassword/).

I'd appreciate feedback if you use this library :-)

## Usage

Add `rpassword` as a dependency in Cargo.toml:

```toml
[dependencies]
rpassword = "3.0.1"
```

Use `rpassword` within your code:

```rust
extern crate rpassword;

fn main() {
    // Prompt for a password on TTY (safest but not default for backwards compatibility)
    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    println!("Your password is {}", pass);
    
    // Prompt for a password on STDOUT
    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    println!("Your password is {}", pass);

    // Prompt for a password on STDERR
    let pass = rpassword::prompt_password_stderr("Password: ").unwrap();
    println!("Your password is {}", pass);

    // Read a password without prompt
    let pass = rpassword::read_password().unwrap();
    println!("Your password is {}", pass);
}
```

## Contributors


We welcome contribution from everyone. Feel free to open an issue or a pull request at any time.

Here's a list of existing `rpassword` contributors:

* [@C4K3](https://github.com/C4K3)
* [@conradkleinespel](https://github.com/conradkleinespel)
* [@DaveLancaster](https://github.com/DaveLancaster)
* [@dcuddeback](https://github.com/dcuddeback)
* [@equalsraf](https://github.com/equalsraf)
* [@JanLikar](https://github.com/JanLikar)
* [@joshuef](https://github.com/joshuef)
* [@longshorej](https://github.com/longshorej)
* [@petevine](https://github.com/petevine)
* [@psych0d0g](https://github.com/psych0d0g)
* [@retep998](https://github.com/retep998)
* [@steveatinfincia](https://github.com/steveatinfincia)
* [@teythoon](https://github.com/teythoon)

Thank you very much for your help!  :smiley:  :heart:
