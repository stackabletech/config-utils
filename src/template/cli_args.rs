use std::path::PathBuf;

use clap::Parser;

use crate::file_types::ReplaceTargetType;

/// Fill out variables in config files from either env variables or files directly.
#[derive(Debug, Parser)]
pub struct TemplateCommand {
    /// The optional path to the file that should be templated. If this is
    /// not specified the utility will try to resolve "nested" environment
    /// variables as opposed to operate on files.
    pub file: Option<PathBuf>,

    /// The optional file type of the file to be templated. If this is not specified this utility will try to infer
    /// the type based on the file name.
    #[arg(value_enum)]
    pub file_type: Option<ReplaceTargetType>,

    /// By default inserted values are automatically escaped according to the deteced file format. You can disable
    /// this, e.g. when you need to insert XML tags (as they otherwise would be escaped).
    /// NOTE: Please make sure to correctly escape the inserted text on your own!
    #[clap(long)]
    pub dont_escape: bool,
}
