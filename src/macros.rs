#[macro_export]
macro_rules! Fatal {
    ($logger:expr, $msg:expr) => {
        $logger.log($msg, Level::Fatal);
    };
}

#[macro_export]
macro_rules! Error {
    ($logger:expr, $msg:expr) => {
        $logger.log($msg, Level::Error);
    };
}

#[macro_export]
macro_rules! Warn {
    ($logger:expr, $msg:expr) => {
        $logger.log($msg, Level::Warn);
    };
}

#[macro_export]
macro_rules! Notice {
    ($logger:expr, $msg:expr) => {
        $logger.log($msg, Level::Notice);
    };
}

#[macro_export]
macro_rules! Info {
    ($logger:expr, $msg:expr) => {
        $logger.log($msg, Level::Info);
    };
}

#[macro_export]
macro_rules! Debug {
    ($logger:expr, $msg:expr) => {
        $logger.log($msg, Level::Debug);
    };
}

#[macro_export]
macro_rules! Trace {
    ($logger:expr, $msg:expr) => {
        $logger.log($msg, Level::Trace);
    };
}
