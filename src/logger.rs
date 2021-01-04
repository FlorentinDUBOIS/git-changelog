//! # Logger module.
//!
//! The logger module provides the log facility.
use std::cmp::min;

use slog::{slog_o, Drain, Level, LevelFilter, Logger};
use slog_async::Async;
use slog_scope::{set_global_logger, GlobalLoggerGuard as Guard};
use slog_term::{FullFormat, TermDecorator};

/// Initialize the logger. Set the verbosity.
#[must_use]
pub fn initialize(verbose: usize) -> Guard {
    let term_decorator = TermDecorator::new().build();
    let term_drain = FullFormat::new(term_decorator).build().fuse();
    let term_drain = Async::new(term_drain).build().fuse();

    let level = Level::Critical.as_usize() + verbose;
    let level = min(level, Level::Trace.as_usize());
    let level = Level::from_usize(level).unwrap_or(Level::Info);

    let drain = LevelFilter::new(term_drain, level).fuse();
    let guard = set_global_logger(Logger::root(drain, slog_o!()));

    guard
}
