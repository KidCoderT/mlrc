use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ArgsManager {
    /// the data file path
    pub data_path: std::path::PathBuf,

    /// the column index that the model should predict
    pub prediction_column: usize,
    
    /// optional the delimiter for the csv file (defaults to ,)
    pub delimiter: Option<String>,

    #[clap(subcommand)]
    pub model: Models,
}

#[derive(Debug, Subcommand)]
pub enum Models {
    /// creates a classification decision tree model
    DTC(DTCInput),

    /// creates a regression decision tree model
    DTR(DTRInput),

    /// creates a classification random forest model
    RFC(RFCInput),

    /// creates a regression random forest model
    RFR(RFRInput),
}

#[derive(Debug, Args)]
pub struct DTCInput {} // todo: implement

#[derive(Debug, Args)]
pub struct DTRInput {} // todo: implement

#[derive(Debug, Args)]
pub struct RFCInput {} // todo: implement

#[derive(Debug, Args)]
pub struct RFRInput {} // todo: implement
