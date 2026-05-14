use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct CommonFinishFlags {
    /// Fetch from origin before performing finish
    #[arg(short = 'F', long)]
    pub fetch: bool,

    /// Keep branch after performing finish
    #[arg(short = 'k', long)]
    pub keep: bool,
}

#[derive(Args, Debug, Clone)]
pub struct TaggingFlags {
    /// Sign the tag cryptographically (GPG)
    #[arg(short = 's', long)]
    pub sign: bool,

    /// Use the given tag message
    #[arg(short = 'm', long)]
    pub message: Option<String>,

    /// Don't tag this release
    #[arg(short = 'n', long)]
    pub notag: bool,
}
