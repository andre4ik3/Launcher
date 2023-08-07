// Copyright © 2023 andre4ik3
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::backtrace::Backtrace;
use std::io::Write;
use std::{fs, io, panic, thread};

use tracing::{error, Level};
use tracing_appender::non_blocking::WorkerGuard;

use crate::directories;

const LOG_FILE_NAME: &str = "Launcher.log";

fn panic_hook(panic_info: &panic::PanicInfo) {
    let location = panic_info.location().map(ToString::to_string);
    let location = location.unwrap_or("<???>".to_string());

    let payload = panic_info.payload();
    let payload = payload
        .downcast_ref::<&str>()
        .map(|str| &**str)
        .or(payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("<unknown error>");

    let thread = thread::current();
    let thread = thread.name().unwrap_or("<unknown>");

    error!("An unrecoverable error has occurred. The application will now terminate.");
    error!("Thread '{thread}' panicked at {location}: {payload}");

    let backtrace = Backtrace::force_capture();
    for line in backtrace.to_string().split('\n') {
        error!("{line}");
    }

    error!("This is a bug in the Launcher. Please report it!");
    io::stderr().flush().expect("failed to flush stderr");
}

/// Sets up logs and panics to go through the [tracing] framework. If compiled for debug and a tty
/// is attached on stderr, logs will be sent there. Otherwise, they will be sent to the log file.
pub fn setup() -> WorkerGuard {
    let directory = directories::DATA.as_path();
    let is_debug = cfg!(debug_assertions) && atty::is(atty::Stream::Stderr);

    // if in development and stderr is attached, write there. otherwise write to log file
    let (writer, guard) = match is_debug {
        true => tracing_appender::non_blocking(io::stderr()),
        false => {
            let _ = fs::remove_file(directory.join(LOG_FILE_NAME));
            let file_appender = tracing_appender::rolling::never(directory, LOG_FILE_NAME);
            tracing_appender::non_blocking(file_appender)
        }
    };

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(writer)
        .with_ansi(is_debug)
        .init();

    panic::set_hook(Box::new(panic_hook));

    // it is important to return the guard, upon dropping the guard, pending logs will be flushed
    guard
}
