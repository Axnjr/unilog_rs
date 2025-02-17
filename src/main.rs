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

    // instance.log("FATAL TRIAL !!", Level::Fatal);
    // instance.log("TRIAL of notoices lets see what happens !!", Level::Notice);
    // instance.log("TRIAL of warnings lets see whta happens !!", Level::Warn);
    // instance.log("TRIAL of informations lets see what hap !!", Level::Info);
    // instance.log("TRIAL of debugging lets see what hap !!", Level::Debug);
    // instance.log("TRIAL of error lets see what hap !!", Level::Error);
    // instance.log("TRIAL of tracing lets see what hap !!", Level::Trace);

    Fatal! (instance, "Trial for Fatal error via macros !!");
    Notice!(instance, "TRIAL of notices lets see what happens !!");
    Warn!  (instance, "TRIAL of warnings lets see what happens !!");
    Info!  (instance, "TRIAL of information lets see what happens !!");
    Debug! (instance, "TRIAL of debugging lets see what happens !!");
    Error! (instance, "TRIAL of error lets see what happens !!");
    Trace! (instance, "TRIAL of tracing lets see what happens !!");
    
}