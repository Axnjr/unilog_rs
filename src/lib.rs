mod macros;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};
use chrono::Local;

#[derive(Clone, Copy, Debug)]
pub enum Level {
    /// If the program cannot continue.  
    Fatal,
    /// I didn't get the expected result, so I'll continue with the other method.  
    Error,
    /// It will be abnormal soon, but there is no problem and you can ignore it.  
    Warn,
    /// It must be enabled in the server production environment.  
    /// Record of passing important points correctly.  
    /// We are monitoring that it is working properly.  
    Notice,
    /// Report highlights.  
    /// Everything that needs to be reported regularly in the production environment.  
    Info,
    /// It should be in a place with many accidents.  
    /// This level is disabled in production environments.  
    /// Leave it in the source and enable it for troubleshooting.  
    /// Often, this is the production level of a desktop operating environment.  
    Debug,
    /// Not included in the distribution.  
    /// Remove this level from the source after using it for debugging.  
    /// If you want to find a bug in the program, write a lot.  
    Trace,
}

impl Level {
    pub(crate) fn color(&self) -> &'static str {
        match self {
            Level::Fatal =>  "\x1b[1m\x1b[31m",   // Red
            Level::Error =>  "\x1b[91m",   // Light Red
            Level::Warn  =>  "\x1b[33m",   // Yellow
            Level::Notice => "\x1b[34m",   // Blue
            Level::Info  =>  "\x1b[32m",   // Green
            Level::Debug =>  "\x1b[35m",   // Magenta
            Level::Trace =>  "\x1b[37m",   // Cyan
        }
    }
}

pub struct UniLog {
    /// by deafult true
    pub log_to_terminal     : bool,
    /// by deafult true
    pub enable_timestamping : bool,
    /// by deafult false
    pub async_logging       : bool,
    pub colorful            : bool,
    pub log_file_name       : Option<String>,
    pub max_log_file_size_in_mb: usize,
    logfile: Option<Arc<Mutex<File>>>,
}


impl UniLog {

    /// Initialize with default values
    pub fn init() -> Self {
        Self {
            log_to_terminal: true,
            enable_timestamping: false,
            async_logging: false,
            colorful: true,
            log_file_name: None,
            max_log_file_size_in_mb: 2000,
            logfile: None,
        }
    }

    pub fn enable_colored_logs(mut self) -> Self {
        self.colorful = true;
        self
    }

    /// Without setting the log file name logs wont be stored in any file !!
    pub fn set_log_file_name(mut self, name: &str) -> Self {
        self.log_file_name = Some(name.to_string());
        // intilize log file
        self.init_file_logging(); 
        self
    }

    /// can be disabled in production enviroment, is set to true by default
    pub fn log_to_terminal(mut self, value: bool) -> Self {
        self.log_to_terminal = value;
        self
    }

    /// Set maximum log file size (MB)
    pub fn max_log_file_size(mut self, size: usize) -> Self {
        self.max_log_file_size_in_mb = size;
        self
    }

    /// Enable timestamping in the log file, disabled by default.
    pub fn enable_timestamping(mut self) -> Self {
        self.enable_timestamping = true;
        self
    }

    /// `Asynchronously` write to the log file and to the terminal.
    /// Use `sync` logging for small applications to `minimize` thread overheads !
    pub fn async_logging(mut self) -> Self {
        self.async_logging = true;
        self
    }

    /// Initialize file logging
    fn init_file_logging(&mut self) {
        if let Some(file_name) = &self.log_file_name {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .write(true)
                .open(file_name)
                .expect("Unable to open log file")
            ;
            self.logfile = Some(Arc::new(Mutex::new(file))); 
        }
    }

    fn write_to_file(log_entry: &str, file_name: &str, file: &mut File, max_log_file_size: usize) {
        // Truncate the file if its size exceed 10MB
        if let Ok(metadata) = file.metadata() {
            if metadata.len() > (max_log_file_size as u64) * 1024 * 1024 {
                println!("Log file exceeded {} MB. Clearing contents.", max_log_file_size);

                *file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(file_name)
                    .expect("Unable to clear log file!")
                ;
            }
        }
        // Write log
        if let Err(e) = file.write_all(log_entry.as_bytes()) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }

    fn log_level_enum_to_string(level: Level) -> String {
        format!("{:<6}", match level {
            Level::Debug =>  "DEBUG",
            Level::Fatal =>  "FATAL",
            Level::Error =>  "ERROR",
            Level::Warn =>   "WARNN",
            Level::Notice => "NOTICE",
            Level::Info =>   "INFO",
            Level::Trace =>  "TRACE",
        })
    }

    /// Write log message
    pub fn log(&self, message: &str, level: Level) {

        let timestamp = Local::now().format("%b %d %H:%M:%S").to_string();
        let level_str = UniLog::log_level_enum_to_string(level);
        let log_entry = format!("[{}] | [{}]   : {} \n", timestamp, level_str, message);

        if let Some(file_name) = &self.log_file_name {

            if let Some(logfile) = &self.logfile {

                // Async Logging
                if self.async_logging {

                    // Clone Arc reference
                    let logfile = Arc::clone(logfile); 
                    let file_name = file_name.clone();
                    let log_entry = log_entry.clone();
                    let mlfs = self.max_log_file_size_in_mb;

                    std::thread::spawn(move || {
                        let mut file = logfile.lock().unwrap();
                        UniLog::write_to_file(&log_entry, &file_name, &mut file, mlfs);
                    });
                    
                } 

                // Sync Logging
                else {
                    let mut file = logfile.lock().unwrap();
                    UniLog::write_to_file(&log_entry, &file_name, &mut file, self.max_log_file_size_in_mb);
                }
            }
        }

        if self.log_to_terminal {
            // for colorful logs 
            if self.colorful {
                println!("{}[{}] | [{}]   : {}", level.color(), timestamp, level_str, message);
            }
            // for simple logs
            else {
                println!("[{}] | [{}]   : {}", timestamp, level_str, message);
            }
        }

    }


}