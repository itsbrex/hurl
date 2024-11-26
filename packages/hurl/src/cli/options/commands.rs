/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
// Generated by bin/spec/options/generate_source.py - Do not modify

pub fn input_files() -> clap::Arg {
    clap::Arg::new("input_files")
        .value_name("FILES")
        .help("Set the input file to use")
        .required(false)
        .index(1)
        .num_args(1..)
}

pub fn aws_sigv4() -> clap::Arg {
    clap::Arg::new("aws_sigv4")
        .long("aws-sigv4")
        .value_name("PROVIDER1[:PROVIDER2[:REGION[:SERVICE]]]")
        .help("Use AWS V4 signature authentication in the transfer")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn cacert_file() -> clap::Arg {
    clap::Arg::new("cacert_file")
        .long("cacert")
        .value_name("FILE")
        .help("CA certificate to verify peer against (PEM format)")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn client_cert_file() -> clap::Arg {
    clap::Arg::new("client_cert_file")
        .long("cert")
        .short('E')
        .value_name("CERTIFICATE[:PASSWORD]")
        .help("Client certificate file and password")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn client_key_file() -> clap::Arg {
    clap::Arg::new("client_key_file")
        .long("key")
        .value_name("KEY")
        .help("Private key file name")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn color() -> clap::Arg {
    clap::Arg::new("color")
        .long("color")
        .help("Colorize output")
        .help_heading("Output options")
        .conflicts_with("no_color")
        .action(clap::ArgAction::SetTrue)
}

pub fn compressed() -> clap::Arg {
    clap::Arg::new("compressed")
        .long("compressed")
        .help("Request compressed response (using deflate or gzip)")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn connect_timeout() -> clap::Arg {
    clap::Arg::new("connect_timeout")
        .long("connect-timeout")
        .value_name("SECONDS")
        .default_value("300")
        .help("Maximum time allowed for connection")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn connect_to() -> clap::Arg {
    clap::Arg::new("connect_to")
        .long("connect-to")
        .value_name("HOST1:PORT1:HOST2:PORT2")
        .help("For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead")
        .help_heading("HTTP options")
        .num_args(1)
        .action(clap::ArgAction::Append)
}

pub fn continue_on_error() -> clap::Arg {
    clap::Arg::new("continue_on_error")
        .long("continue-on-error")
        .help("Continue executing requests even if an error occurs")
        .help_heading("Run options")
        .action(clap::ArgAction::SetTrue)
}

pub fn cookies_input_file() -> clap::Arg {
    clap::Arg::new("cookies_input_file")
        .long("cookie")
        .short('b')
        .value_name("FILE")
        .help("Read cookies from FILE")
        .help_heading("Other options")
        .num_args(1)
}

pub fn cookies_output_file() -> clap::Arg {
    clap::Arg::new("cookies_output_file")
        .long("cookie-jar")
        .short('c')
        .value_name("FILE")
        .help("Write cookies to FILE after running the session (only for one session)")
        .help_heading("Other options")
        .num_args(1)
}

pub fn curl() -> clap::Arg {
    clap::Arg::new("curl")
        .long("curl")
        .value_name("FILE")
        .help("Export each request to a list of curl commands")
        .help_heading("Output options")
        .num_args(1)
}

pub fn delay() -> clap::Arg {
    clap::Arg::new("delay")
        .long("delay")
        .value_name("MILLISECONDS")
        .default_value("0")
        .help("Sets delay before each request")
        .help_heading("Run options")
        .num_args(1)
}

pub fn error_format() -> clap::Arg {
    clap::Arg::new("error_format")
        .long("error-format")
        .value_name("FORMAT")
        .default_value("short")
        .value_parser(["short", "long"])
        .help("Control the format of error messages")
        .help_heading("Output options")
        .num_args(1)
}

pub fn file_root() -> clap::Arg {
    clap::Arg::new("file_root")
        .long("file-root")
        .value_name("DIR")
        .help("Set root directory to import files [default: input file directory]")
        .help_heading("Other options")
        .num_args(1)
}

pub fn follow_location() -> clap::Arg {
    clap::Arg::new("follow_location")
        .long("location")
        .short('L')
        .help("Follow redirects")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn follow_location_trusted() -> clap::Arg {
    clap::Arg::new("follow_location_trusted")
        .long("location-trusted")
        .help("Follow redirects but allows sending the name + password to all hosts that the site may redirect to")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn from_entry() -> clap::Arg {
    clap::Arg::new("from_entry")
        .long("from-entry")
        .value_name("ENTRY_NUMBER")
        .value_parser(clap::value_parser!(u32).range(1..))
        .help("Execute Hurl file from ENTRY_NUMBER (starting at 1)")
        .help_heading("Run options")
        .conflicts_with("interactive")
        .num_args(1)
}

pub fn glob() -> clap::Arg {
    clap::Arg::new("glob")
        .long("glob")
        .value_name("GLOB")
        .help("Specify input files that match the given GLOB. Multiple glob flags may be used")
        .help_heading("Other options")
        .num_args(1)
        .action(clap::ArgAction::Append)
}

pub fn http10() -> clap::Arg {
    clap::Arg::new("http10")
        .long("http1.0")
        .short('0')
        .help("Tell Hurl to use HTTP version 1.0")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn http11() -> clap::Arg {
    clap::Arg::new("http11")
        .long("http1.1")
        .help("Tell Hurl to use HTTP version 1.1")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn http2() -> clap::Arg {
    clap::Arg::new("http2")
        .long("http2")
        .help("Tell Hurl to use HTTP version 2")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn http3() -> clap::Arg {
    clap::Arg::new("http3")
        .long("http3")
        .help("Tell Hurl to use HTTP version 3")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn ignore_asserts() -> clap::Arg {
    clap::Arg::new("ignore_asserts")
        .long("ignore-asserts")
        .help("Ignore asserts defined in the Hurl file")
        .help_heading("Run options")
        .action(clap::ArgAction::SetTrue)
}

pub fn include() -> clap::Arg {
    clap::Arg::new("include")
        .long("include")
        .short('i')
        .help("Include the HTTP headers in the output")
        .help_heading("Output options")
        .action(clap::ArgAction::SetTrue)
}

pub fn insecure() -> clap::Arg {
    clap::Arg::new("insecure")
        .long("insecure")
        .short('k')
        .help("Allow insecure SSL connections")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn interactive() -> clap::Arg {
    clap::Arg::new("interactive")
        .long("interactive")
        .help("Turn on interactive mode")
        .help_heading("Run options")
        .conflicts_with("to_entry")
        .action(clap::ArgAction::SetTrue)
}

pub fn ipv4() -> clap::Arg {
    clap::Arg::new("ipv4")
        .long("ipv4")
        .short('4')
        .help("Tell Hurl to use IPv4 addresses only when resolving host names, and not for example try IPv6")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn ipv6() -> clap::Arg {
    clap::Arg::new("ipv6")
        .long("ipv6")
        .short('6')
        .help("Tell Hurl to use IPv6 addresses only when resolving host names, and not for example try IPv4")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn jobs() -> clap::Arg {
    clap::Arg::new("jobs")
        .long("jobs")
        .value_name("NUM")
        .value_parser(clap::value_parser!(u32).range(1..))
        .help("Maximum number of parallel jobs")
        .help_heading("Run options")
        .num_args(1)
}

pub fn json() -> clap::Arg {
    clap::Arg::new("json")
        .long("json")
        .help("Output each Hurl file result to JSON")
        .help_heading("Output options")
        .conflicts_with("no_output")
        .action(clap::ArgAction::SetTrue)
}

pub fn limit_rate() -> clap::Arg {
    clap::Arg::new("limit_rate")
        .long("limit-rate")
        .value_name("SPEED")
        .value_parser(clap::value_parser!(u64))
        .help("Specify the maximum transfer rate in bytes/second, for both downloads and uploads")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn max_filesize() -> clap::Arg {
    clap::Arg::new("max_filesize")
        .long("max-filesize")
        .value_name("BYTES")
        .value_parser(clap::value_parser!(u64))
        .help("Specify the maximum size in bytes of a file to download")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn max_redirects() -> clap::Arg {
    clap::Arg::new("max_redirects")
        .long("max-redirs")
        .value_name("NUM")
        .default_value("50")
        .value_parser(clap::value_parser!(i32).range(-1..))
        .allow_hyphen_values(true)
        .help("Maximum number of redirects allowed, -1 for unlimited redirects")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn max_time() -> clap::Arg {
    clap::Arg::new("max_time")
        .long("max-time")
        .short('m')
        .value_name("SECONDS")
        .default_value("300")
        .help("Maximum time allowed for the transfer")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn netrc() -> clap::Arg {
    clap::Arg::new("netrc")
        .long("netrc")
        .short('n')
        .help("Must read .netrc for username and password")
        .help_heading("Other options")
        .conflicts_with("netrc_file")
        .conflicts_with("netrc_optional")
        .action(clap::ArgAction::SetTrue)
}

pub fn netrc_file() -> clap::Arg {
    clap::Arg::new("netrc_file")
        .long("netrc-file")
        .value_name("FILE")
        .help("Specify FILE for .netrc")
        .help_heading("Other options")
        .conflicts_with("netrc")
        .num_args(1)
}

pub fn netrc_optional() -> clap::Arg {
    clap::Arg::new("netrc_optional")
        .long("netrc-optional")
        .help("Use either .netrc or the URL")
        .help_heading("Other options")
        .conflicts_with("netrc")
        .action(clap::ArgAction::SetTrue)
}

pub fn no_color() -> clap::Arg {
    clap::Arg::new("no_color")
        .long("no-color")
        .help("Do not colorize output")
        .help_heading("Output options")
        .conflicts_with("color")
        .action(clap::ArgAction::SetTrue)
}

pub fn no_output() -> clap::Arg {
    clap::Arg::new("no_output")
        .long("no-output")
        .help("Suppress output. By default, Hurl outputs the body of the last response")
        .help_heading("Output options")
        .conflicts_with("json")
        .action(clap::ArgAction::SetTrue)
}

pub fn noproxy() -> clap::Arg {
    clap::Arg::new("noproxy")
        .long("noproxy")
        .value_name("HOST(S)")
        .help("List of hosts which do not use proxy")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn output() -> clap::Arg {
    clap::Arg::new("output")
        .long("output")
        .short('o')
        .value_name("FILE")
        .help("Write to FILE instead of stdout")
        .help_heading("Output options")
        .num_args(1)
}

pub fn parallel() -> clap::Arg {
    clap::Arg::new("parallel")
        .long("parallel")
        .help("Run files in parallel (default in test mode)")
        .help_heading("Run options")
        .action(clap::ArgAction::SetTrue)
}

pub fn path_as_is() -> clap::Arg {
    clap::Arg::new("path_as_is")
        .long("path-as-is")
        .help("Tell Hurl to not handle sequences of /../ or /./ in the given URL path")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn proxy() -> clap::Arg {
    clap::Arg::new("proxy")
        .long("proxy")
        .short('x')
        .value_name("[PROTOCOL://]HOST[:PORT]")
        .help("Use proxy on given PROTOCOL/HOST/PORT")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn repeat() -> clap::Arg {
    clap::Arg::new("repeat")
        .long("repeat")
        .value_name("NUM")
        .value_parser(clap::value_parser!(i32).range(-1..))
        .allow_hyphen_values(true)
        .help("Repeat the input files sequence NUM times, -1 for infinite loop")
        .help_heading("Run options")
        .num_args(1)
}

pub fn report_html() -> clap::Arg {
    clap::Arg::new("report_html")
        .long("report-html")
        .value_name("DIR")
        .help("Generate HTML report to DIR")
        .help_heading("Report options")
        .num_args(1)
}

pub fn report_json() -> clap::Arg {
    clap::Arg::new("report_json")
        .long("report-json")
        .value_name("DIR")
        .help("Generate JSON report to DIR")
        .help_heading("Report options")
        .num_args(1)
}

pub fn report_junit() -> clap::Arg {
    clap::Arg::new("report_junit")
        .long("report-junit")
        .value_name("FILE")
        .help("Write a JUnit XML report to FILE")
        .help_heading("Report options")
        .num_args(1)
}

pub fn report_tap() -> clap::Arg {
    clap::Arg::new("report_tap")
        .long("report-tap")
        .value_name("FILE")
        .help("Write a TAP report to FILE")
        .help_heading("Report options")
        .num_args(1)
}

pub fn resolve() -> clap::Arg {
    clap::Arg::new("resolve")
        .long("resolve")
        .value_name("HOST:PORT:ADDR")
        .help("Provide a custom address for a specific HOST and PORT pair")
        .help_heading("HTTP options")
        .num_args(1)
        .action(clap::ArgAction::Append)
}

pub fn retry() -> clap::Arg {
    clap::Arg::new("retry")
        .long("retry")
        .value_name("NUM")
        .value_parser(clap::value_parser!(i32).range(-1..))
        .allow_hyphen_values(true)
        .help("Maximum number of retries, 0 for no retries, -1 for unlimited retries")
        .help_heading("Run options")
        .num_args(1)
}

pub fn retry_interval() -> clap::Arg {
    clap::Arg::new("retry_interval")
        .long("retry-interval")
        .value_name("MILLISECONDS")
        .default_value("1000")
        .help("Interval in milliseconds before a retry")
        .help_heading("Run options")
        .num_args(1)
}

pub fn ssl_no_revoke() -> clap::Arg {
    clap::Arg::new("ssl_no_revoke")
        .long("ssl-no-revoke")
        .help("(Windows) Tell Hurl to disable certificate revocation checks")
        .help_heading("HTTP options")
        .action(clap::ArgAction::SetTrue)
}

pub fn test() -> clap::Arg {
    clap::Arg::new("test")
        .long("test")
        .help("Activate test mode (use parallel execution)")
        .help_heading("Run options")
        .action(clap::ArgAction::SetTrue)
}

pub fn to_entry() -> clap::Arg {
    clap::Arg::new("to_entry")
        .long("to-entry")
        .value_name("ENTRY_NUMBER")
        .value_parser(clap::value_parser!(u32).range(1..))
        .help("Execute Hurl file to ENTRY_NUMBER (starting at 1)")
        .help_heading("Run options")
        .conflicts_with("interactive")
        .num_args(1)
}

pub fn unix_socket() -> clap::Arg {
    clap::Arg::new("unix_socket")
        .long("unix-socket")
        .value_name("PATH")
        .help("(HTTP) Connect through this Unix domain socket, instead of using the network")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn user() -> clap::Arg {
    clap::Arg::new("user")
        .long("user")
        .short('u')
        .value_name("USER:PASSWORD")
        .help("Add basic Authentication header to each request")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn user_agent() -> clap::Arg {
    clap::Arg::new("user_agent")
        .long("user-agent")
        .short('A')
        .value_name("NAME")
        .help("Specify the User-Agent string to send to the HTTP server")
        .help_heading("HTTP options")
        .num_args(1)
}

pub fn variable() -> clap::Arg {
    clap::Arg::new("variable")
        .long("variable")
        .value_name("NAME=VALUE")
        .help("Define a variable")
        .help_heading("Run options")
        .num_args(1)
        .action(clap::ArgAction::Append)
}

pub fn variables_file() -> clap::Arg {
    clap::Arg::new("variables_file")
        .long("variables-file")
        .value_name("FILE")
        .help("Define a properties file in which you define your variables")
        .help_heading("Run options")
        .num_args(1)
        .action(clap::ArgAction::Append)
}

pub fn verbose() -> clap::Arg {
    clap::Arg::new("verbose")
        .long("verbose")
        .short('v')
        .help("Turn on verbose")
        .help_heading("Output options")
        .action(clap::ArgAction::SetTrue)
}

pub fn very_verbose() -> clap::Arg {
    clap::Arg::new("very_verbose")
        .long("very-verbose")
        .help("Turn on verbose output, including HTTP response and libcurl logs")
        .help_heading("Output options")
        .action(clap::ArgAction::SetTrue)
}
