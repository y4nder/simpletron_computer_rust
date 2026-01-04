use clap::Parser;
#[derive(Parser, Debug)]
#[command(name = "simpletron", version, about = "A virtual machine")]
pub struct CliArgs {
    /// Path to the sml file
    pub filename: String,

    /// Optional debugger to view the state of the memory and the cpu
    #[arg(long)]
    pub debug: bool,
}
