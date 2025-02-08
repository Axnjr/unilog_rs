# `🦀unilog.rs`

## Example
```rust
use unilog::{Level, UniLog, Fatal, Error, Warn, Notice, Info, Debug, Trace};

fn main() {
    let instance = UniLog::init()
        .set_log_file_name("server.log")
        .log_to_terminal(true)
        .async_logging()
        .enable_timestamping()
        .enable_colored_logs()
        .max_log_file_size(2000)
    ;

    Fatal! (instance, "Trial for Fatal error via macros !!");
    Notice!(instance, "TRIAL of notices lets see what happens !!");
    Warn!  (instance, "TRIAL of warnings lets see what happens !!");
    Info!  (instance, "TRIAL of information lets see what happens !!");
    Debug! (instance, "TRIAL of debugging lets see what happens !!");
    Error! (instance, "TRIAL of error lets see what happens !!");
    Trace! (instance, "TRIAL of tracing lets see what happens !!");
}
```
