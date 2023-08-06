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

use std::{fs, io};

use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

pub mod directories;

const LOG_FILE_NAME: &str = "Launcher.log";

pub fn setup_logging() -> (WorkerGuard, WorkerGuard) {
    let directory = directories::DATA.as_path();

    // clean-up previous log file
    let _ = fs::remove_file(directory.join(LOG_FILE_NAME));

    // create non-blocking file appender (this will also create the file)
    let file_appender = tracing_appender::rolling::never(directory, LOG_FILE_NAME);
    let (file, guard_file) = tracing_appender::non_blocking(file_appender);

    // file writer
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_ansi(false)
        .with_writer(file);
    // .init();

    // stderr writer
    let (stderr, guard_stderr) = tracing_appender::non_blocking(io::stderr());
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_target(false)
        .with_writer(stderr)
        .init();

    // it is important to return the guard, upon dropping the guard, pending logs will be flushed
    (guard_stderr, guard_file)
}
