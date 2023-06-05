// SPDX-License-Identifier: MIT OR Apache-2.0
// This file is part of Static Web Server.
// See https://static-web-server.net/ for more information
// Copyright (C) 2019-present Jose Quintana <joseluisq.net>

//! The server CLI options

use clap::Parser;
use std::path::PathBuf;

use crate::directory_listing::DirListFmt;

/// General server configuration available in CLI and config file options.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct General {
    #[arg(long, short = 'a', default_value = "::", env = "SERVER_HOST")]
    /// Host address (E.g 127.0.0.1 or ::1)
    pub host: String,

    #[arg(long, short = 'p', default_value = "80", env = "SERVER_PORT")]
    /// Host port
    pub port: u16,

    #[arg(
        long,
        short = 'f',
        env = "SERVER_LISTEN_FD",
        conflicts_with_all(&["host", "port", "https_redirect"])
    )]
    /// Instead of binding to a TCP port, accept incoming connections to an already-bound TCP
    /// socket listener on the specified file descriptor number (usually zero). Requires that the
    /// parent process (e.g. inetd, launchd, or systemd) binds an address and port on behalf of
    /// static-web-server, before arranging for the resulting file descriptor to be inherited by
    /// static-web-server. Cannot be used in conjunction with the port and host arguments. The
    /// included systemd unit file utilises this feature to increase security by allowing the
    /// static-web-server to be sandboxed more completely.
    pub fd: Option<usize>,

    #[cfg_attr(
        not(wasm),
        arg(
            long,
            short = 'n',
            default_value = "1",
            env = "SERVER_THREADS_MULTIPLIER"
        )
    )]
    #[cfg_attr(
        wasm,
        arg(
            long,
            short = 'n',
            default_value = "2",
            env = "SERVER_THREADS_MULTIPLIER"
        )
    )] // We use 2 as the threads multiplier in Wasm, 1 in Native
    /// Number of worker threads multiplier that'll be multiplied by the number of system CPUs
    /// using the formula: `worker threads = number of CPUs * n` where `n` is the value that changes here.
    /// When multiplier value is 0 or 1 then one thread per core is used.
    /// Number of worker threads result should be a number between 1 and 32,768 though it is advised to keep this value on the smaller side.
    pub threads_multiplier: usize,

    #[cfg_attr(
        not(wasm),
        arg(
            long,
            short = 'b',
            default_value = "512",
            env = "SERVER_MAX_BLOCKING_THREADS"
        )
    )]
    #[cfg_attr(
        wasm,
        arg(
            long,
            short = 'b',
            default_value = "20",
            env = "SERVER_MAX_BLOCKING_THREADS"
        )
    )] // We use 20 in Wasm, 512 in Native (default for tokio)
    /// Maximum number of blocking threads
    pub max_blocking_threads: usize,

    #[arg(long, short = 'd', default_value = "./public", env = "SERVER_ROOT")]
    /// Root directory path of static files.
    pub root: PathBuf,

    #[arg(
        long,
        default_value = "./public/50x.html",
        env = "SERVER_ERROR_PAGE_50X"
    )]
    /// HTML file path for 50x errors. If the path is not specified or simply doesn't exist then the server will use a generic HTML error message.
    pub page50x: PathBuf,

    #[arg(
        long,
        default_value = "./public/404.html",
        env = "SERVER_ERROR_PAGE_404"
    )]
    /// HTML file path for 404 errors. If the path is not specified or simply doesn't exist then the server will use a generic HTML error message.
    pub page404: PathBuf,

    #[arg(long, env = "SERVER_FALLBACK_PAGE")]
    /// HTML file path that is used for GET requests when the requested path doesn't exist. The fallback page is served with a 200 status code, useful when using client routers. If the path is not specified or simply doesn't exist then this feature will not be active.
    pub page_fallback: Option<PathBuf>,

    #[arg(long, short = 'g', default_value = "error", env = "SERVER_LOG_LEVEL")]
    /// Specify a logging level in lower case. Values: error, warn, info, debug or trace
    pub log_level: String,

    #[arg(
        long,
        short = 'c',
        default_value = "",
        env = "SERVER_CORS_ALLOW_ORIGINS"
    )]
    /// Specify an optional CORS list of allowed origin hosts separated by commas. Host ports or protocols aren't being checked. Use an asterisk (*) to allow any host.
    pub cors_allow_origins: String,

    #[arg(
        long,
        short = 'j',
        default_value = "origin, content-type",
        env = "SERVER_CORS_ALLOW_HEADERS"
    )]
    /// Specify an optional CORS list of allowed headers separated by commas. Default "origin, content-type". It requires `--cors-allow-origins` to be used along with.
    pub cors_allow_headers: String,

    #[arg(
        long,
        default_value = "origin, content-type",
        env = "SERVER_CORS_EXPOSE_HEADERS"
    )]
    /// Specify an optional CORS list of exposed headers separated by commas. Default "origin, content-type". It requires `--cors-expose-origins` to be used along with.
    pub cors_expose_headers: String,

    #[arg(
        long,
        short = 't',
        default_value = "false",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_HTTP2_TLS",
    )]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    /// Enable HTTP/2 with TLS support.
    pub http2: bool,

    #[arg(long, required_if_eq("http2", "true"), env = "SERVER_HTTP2_TLS_CERT")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    /// Specify the file path to read the certificate.
    pub http2_tls_cert: Option<PathBuf>,

    #[arg(long, required_if_eq("http2", "true"), env = "SERVER_HTTP2_TLS_KEY")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    /// Specify the file path to read the private key.
    pub http2_tls_key: Option<PathBuf>,

    #[arg(
        long,
        default_value = "false",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        requires_if("true", "http2"),
        env = "SERVER_HTTPS_REDIRECT"
    )]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    /// Redirect all requests with scheme "http" to "https" for the current server instance. It depends on "http2" to be enabled.
    pub https_redirect: bool,

    #[arg(
        long,
        requires_if("true", "https_redirect"),
        default_value = "localhost",
        env = "SERVER_HTTPS_REDIRECT_HOST"
    )]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    /// Canonical host name or IP of the HTTPS (HTTPS/2) server. It depends on "https_redirect" to be enabled.
    pub https_redirect_host: String,

    #[arg(
        long,
        requires_if("true", "https_redirect"),
        default_value = "80",
        env = "SERVER_HTTPS_REDIRECT_FROM_PORT"
    )]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    /// HTTP host port where the redirect server will listen for requests to redirect them to HTTPS. It depends on "https_redirect" to be enabled.
    pub https_redirect_from_port: u16,

    #[arg(
        long,
        requires_if("true", "https_redirect"),
        default_value = "localhost",
        env = "SERVER_HTTPS_REDIRECT_FROM_HOSTS"
    )]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    /// List of host names or IPs allowed to redirect from. HTTP requests must contain the HTTP 'Host' header and match against this list. It depends on "https_redirect" to be enabled.
    pub https_redirect_from_hosts: String,

    #[cfg(feature = "compression")]
    #[cfg_attr(docsrs, doc(cfg(feature = "compression")))]
    #[arg(
        long,
        short = 'x',
        default_value = "true",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_COMPRESSION",
    )]
    /// Gzip, Deflate, Brotli or Zstd compression on demand determined by the Accept-Encoding header and applied to text-based web file types only.
    pub compression: bool,

    #[cfg(feature = "compression")]
    #[cfg_attr(docsrs, doc(cfg(feature = "compression")))]
    #[arg(
        long,
        default_value = "false",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_COMPRESSION_STATIC",
    )]
    /// Look up the pre-compressed file variant (`.gz`, `.br` or `.zst`) on disk of a requested file and serves it directly if available.
    /// The compression type is determined by the `Accept-Encoding` header.
    pub compression_static: bool,

    #[arg(
        long,
        short = 'z',
        default_value = "false",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_DIRECTORY_LISTING",
    )]
    /// Enable directory listing for all requests ending with the slash character (‘/’).
    pub directory_listing: bool,

    #[arg(
        long,
        requires_if("true", "directory_listing"),
        default_value = "6",
        env = "SERVER_DIRECTORY_LISTING_ORDER"
    )]
    /// Specify a default code number to order directory listing entries per `Name`, `Last modified` or `Size` attributes (columns). Code numbers supported: 0 (Name asc), 1 (Name desc), 2 (Last modified asc), 3 (Last modified desc), 4 (Size asc), 5 (Size desc). Default 6 (unordered)
    pub directory_listing_order: u8,

    #[arg(
        long,
        value_enum,
        requires_if("true", "directory_listing"),
        default_value = "html",
        env = "SERVER_DIRECTORY_LISTING_FORMAT",
        ignore_case(true)
    )]
    /// Specify a content format for directory listing entries. Formats supported: "html" or "json". Default "html".
    pub directory_listing_format: DirListFmt,

    #[arg(
        long,
        default_value = "false",
        default_value_if("http2", "true", Some("true")),
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_SECURITY_HEADERS",
    )]
    /// Enable security headers by default when HTTP/2 feature is activated.
    /// Headers included: "Strict-Transport-Security: max-age=63072000; includeSubDomains; preload" (2 years max-age),
    /// "X-Frame-Options: DENY" and "Content-Security-Policy: frame-ancestors 'self'".
    pub security_headers: bool,

    #[arg(
        long,
        short = 'e',
        default_value = "true",
        env = "SERVER_CACHE_CONTROL_HEADERS"
    )]
    #[arg(
        long,
        short = 'e',
        default_value = "true",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_CACHE_CONTROL_HEADERS",
    )]
    /// Enable cache control headers for incoming requests based on a set of file types. The file type list can be found on `src/control_headers.rs` file.
    pub cache_control_headers: bool,

    /// It provides The "Basic" HTTP Authentication scheme using credentials as "user-id:password" pairs. Password must be encoded using the "BCrypt" password-hashing function.
    #[arg(long, default_value = "", env = "SERVER_BASIC_AUTH")]
    pub basic_auth: String,

    #[arg(long, short = 'q', default_value = "0", env = "SERVER_GRACE_PERIOD")]
    /// Defines a grace period in seconds after a `SIGTERM` signal is caught which will delay the server before to shut it down gracefully. The maximum value is 255 seconds.
    pub grace_period: u8,

    #[arg(long, short = 'w', env = "SERVER_CONFIG_FILE")]
    /// Server TOML configuration file path.
    pub config_file: Option<PathBuf>,

    #[arg(
        long,
        default_value = "false",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_LOG_REMOTE_ADDRESS",
    )]
    /// Log incoming requests information along with its remote address if available using the `info` log level.
    pub log_remote_address: bool,

    #[arg(
        long,
        default_value = "true",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_REDIRECT_TRAILING_SLASH",
    )]
    /// Check for a trailing slash in the requested directory URI and redirect permanently (308) to the same path with a trailing slash suffix if it is missing.
    pub redirect_trailing_slash: bool,

    #[arg(
        long,
        default_value = "false",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_IGNORE_HIDDEN_FILES",
    )]
    /// Ignore hidden files/directories (dotfiles), preventing them to be served and being included in auto HTML index pages (directory listing).
    pub ignore_hidden_files: bool,

    //
    // Windows specific arguments and commands
    //
    #[cfg(windows)]
    #[arg(
        long,
        short = 's',
        default_value = "false",
        default_missing_value("true"),
        num_args(0..=1),
        require_equals(true),
        action = clap::ArgAction::Set,
        env = "SERVER_WINDOWS_SERVICE",
    )]
    /// Tell the web server to run in a Windows Service context. Note that the `install` subcommand will enable this option automatically.
    pub windows_service: bool,

    // Windows commands
    #[cfg(windows)]
    #[command(subcommand)]
    /// Subcommands to install or uninstall the SWS Windows Service.
    pub commands: Option<Commands>,
}

#[cfg(windows)]
#[derive(Debug, clap::Subcommand)]
/// Subcommands to install or uninstall the SWS Windows Service.
pub enum Commands {
    /// Install a Windows Service for the web server.
    #[command(name = "install")]
    Install {},

    /// Uninstall the current Windows Service.
    #[command(name = "uninstall")]
    Uninstall {},
}
