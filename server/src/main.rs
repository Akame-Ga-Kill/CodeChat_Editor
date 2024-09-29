/// Copyright (C) 2023 Bryan A. Jones.
///
/// This file is part of the CodeChat Editor. The CodeChat Editor is free
/// software: you can redistribute it and/or modify it under the terms of the
/// GNU General Public License as published by the Free Software Foundation,
/// either version 3 of the License, or (at your option) any later version.
///
/// The CodeChat Editor is distributed in the hope that it will be useful, but
/// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
/// or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
/// more details.
///
/// You should have received a copy of the GNU General Public License along with
/// the CodeChat Editor. If not, see
/// [http://www.gnu.org/licenses](http://www.gnu.org/licenses).
///
/// # `main.rs` -- Entrypoint for the CodeChat Editor Server
use code_chat_editor::webserver;
use actix_web::main as actix_main;  // Import the Actix runtime's main function

// The #[cfg(not(tarpaulin_include))] attribute ensures that this code
// is not included in the test coverage measurement using tarpaulin (a code coverage tool).
#[cfg(not(tarpaulin_include))]
// The #[actix_main] macro replaces the default Rust main function with an asynchronous runtime for Actix.
#[actix_main]
async fn main() -> std::io::Result<()> {
    // Configure the logger using log4rs, which reads from a configuration file to initialize logging.
    webserver::configure_logger();
    
    // Call the main server function from webserver.rs and await its result.
    // This function starts the HTTP server, dynamically selects an available port, and listens for incoming requests.
    webserver::main().await
}

