# `🦀unilog.rs`

## Example
```rust
use unilog::{Level, UniLog, Fatal, Error, Warn, Notice, Info, Debug, Trace};

fn main() {
    let instance = UniLog::init()
        .set_log_file_name("server.log") // dont forget the file extension
        .log_to_terminal(true)
        .async_logging()
        .enable_timestamping()
        .enable_colored_logs()
        .max_log_file_size(20) // 20 Mb, would truncate the log file if its size exceeds this limit!
    ;

    Fatal! (instance, "TRIAL of fatal error via macros !!");
    Notice!(instance, "TRIAL of notices lets see what happens !!");
    Warn!  (instance, "TRIAL of warnings lets see what happens !!");
    Info!  (instance, "TRIAL of information lets see what happens !!");
    Debug! (instance, "TRIAL of debugging lets see what happens !!");
    Error! (instance, "TRIAL of error lets see what happens !!");
    Trace! (instance, "TRIAL of tracing lets see what happens !!");
}
```

# Sample logs
```py
[Feb 08 16:38:33] | [NOTICE]   : TRIAL of notoices lets see what happens !! 
[Feb 08 16:38:33] | [WARNN ]   : TRIAL of warnings lets see whta happens !! 
[Feb 08 16:38:33] | [ERROR ]   : TRIAL of error lets see what hap !! 
[Feb 08 16:38:33] | [TRACE ]   : TRIAL of tracing lets see what hap !! 
[Feb 08 16:44:05] | [FATAL ]   : Trial for Fatal error via macros !! 
[Feb 08 16:46:04] | [FATAL ]   : Trial for Fatal error via macros !! 
[Feb 08 16:46:04] | [NOTICE]   : TRIAL of notices lets see what happens !! 
[Feb 08 16:46:04] | [WARNN ]   : TRIAL of warnings lets see what happens !! 
[Feb 08 16:46:04] | [INFO  ]   : TRIAL of information lets see what happens !! 
[Feb 08 16:46:04] | [DEBUG ]   : TRIAL of debugging lets see what happens !! 
[Feb 08 16:46:04] | [ERROR ]   : TRIAL of error lets see what happens !! 
[Feb 08 16:46:04] | [TRACE ]   : TRIAL of tracing lets see what happens !! 
```
