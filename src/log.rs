use slog;
use slog_async;
use slog_term;
use slog::Drain;

// TODO: Being able to change log levels at runtime would be nice.
// The Right Way to do that might be to use different loggers though,
// rather than alter the global one.
lazy_static! {
    pub static ref LOG: slog::Logger = {
        let decorator = slog_term::TermDecorator::new().build();;
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let log = slog::Logger::root(drain, o!());
        debug!(log, "Logging started");
        log
    };
}
