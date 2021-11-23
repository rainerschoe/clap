use std::str::FromStr;


/// Provides hints about argument types for shell command completion.
///
/// See the `clap_generate` crate for completion script generation.
///
/// Overview of which hints are supported by which shell:
///
/// | Hint                   | zsh | fish[^1]|
/// | ---------------------- | --- | ------- |
/// | `AnyPath`              | Yes | Yes     |
/// | `FilePath`             | Yes | Yes     |
/// | `DirPath`              | Yes | Yes     |
/// | `ExecutablePath`       | Yes | Partial |
/// | `CommandName`          | Yes | Yes     |
/// | `CommandString`        | Yes | Partial |
/// | `CommandWithArguments` | Yes |         |
/// | `Username`             | Yes | Yes     |
/// | `Hostname`             | Yes | Yes     |
/// | `Url`                  | Yes |         |
/// | `EmailAddress`         | Yes |         |
/// | `DynamicValues`        |     | Yes     |
///
/// [^1]: fish completions currently only support named arguments (e.g. -o or --opt), not
///       positional arguments.
// DISCUSS: Copy Trait
// DISCUSS: Fancy rust enum
#[derive(Debug, PartialEq, Clone)]
pub enum ValueHint {
    /// Default value if hint is not specified. Follows shell default behavior, which is usually
    /// auto-completing filenames.
    Unknown,
    /// None of the hints below apply. Disables shell completion for this argument.
    Other,
    /// Any existing path.
    AnyPath,
    /// Path to a file.
    FilePath,
    /// Path to a directory.
    DirPath,
    /// Path to an executable file.
    ExecutablePath,
    /// Name of a command, without arguments. May be relative to PATH, or full path to executable.
    CommandName,
    /// A single string containing a command and its arguments.
    CommandString,
    /// Capture the remaining arguments as a command name and arguments for that command. This is
    /// common when writing shell wrappers that execute anther command, for example `sudo` or `env`.
    ///
    /// This hint is special, the argument must be a positional argument and have
    /// [`.multiple_values(true)`] and App must use [`AppSettings::TrailingVarArg`]. The result is that the
    /// command line `my_app ls -la /` will be parsed as `["ls", "-la", "/"]` and clap won't try to
    /// parse the `-la` argument itself.
    ///
    /// [`AppSettings::TrailingVarArg`]: crate::AppSettings::TrailingVarArg
    /// [`.multiple_values(true)`]: crate::Arg::multiple_values()
    CommandWithArguments,
    /// Name of a local operating system user.
    Username,
    /// Host name of a computer.
    /// Shells usually parse `/etc/hosts` and `.ssh/known_hosts` to complete hostnames.
    Hostname,
    /// Complete web address.
    Url,
    /// Email address.
    EmailAddress,

    /// Values which are to be retrieved dynamically at runtime by calling an external command.
    /// The given string is the command which will be executed at runtime to generate a list of
    /// possible values.
    /// This is useful if values depend on external services or user input.
    /// For example a list of nearby cities or files on a remote server.
    ///
    /// The given command will get all the arguments which are given to the executable.
    /// It is expected to return a list of possible values separated by newline `\n`.
    /// Optionally each possible value may also contain a documentation string
    /// separated by a tabulator `\t`. Depending on the shell, this will be used
    /// to display documentation.
    ///
    /// Example:
    ///     1. DynamicValue Hint is set to `helloCity --getNearbyCities`
    ///
    ///     2.User types: `helloCity --zipcode 12345 --city ` + TAB
    ///
    ///     3. The following will be evaluated in the shell to get a list of possible values:
    ///        `helloCity --getNearbyCities --zipcode 12345 --city`
    ///        The stdout output  of the above command should be something like this:
    ///        ```
    ///        Berlin \t Capital of Germany
    ///        Potsdam \t City near Berlin
    ///        Spandau \t City with lots of waterways
    ///        ```
    ///     4. This will result in three possible values being used for completion
    DynamicValues(String)
}

impl Default for ValueHint {
    fn default() -> Self {
        ValueHint::Unknown
    }
}

impl FromStr for ValueHint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match &*s.to_ascii_lowercase() {
            "unknown" => ValueHint::Unknown,
            "other" => ValueHint::Other,
            "anypath" => ValueHint::AnyPath,
            "filepath" => ValueHint::FilePath,
            "dirpath" => ValueHint::DirPath,
            "executablepath" => ValueHint::ExecutablePath,
            "commandname" => ValueHint::CommandName,
            "commandstring" => ValueHint::CommandString,
            "commandwitharguments" => ValueHint::CommandWithArguments,
            "username" => ValueHint::Username,
            "hostname" => ValueHint::Hostname,
            "url" => ValueHint::Url,
            "emailaddress" => ValueHint::EmailAddress,
            _ => return Err(format!("unknown ValueHint: `{}`", s)),
        })
    }
}
