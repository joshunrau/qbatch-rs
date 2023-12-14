mod config;

use clap::{ArgAction, Parser};
use std::path::PathBuf;

use config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None )]
pub struct Arguments {
    /** An input file containing a list of shell commands to be submitted. Alternatively, use - to read the command list from stdin or -- followed by a single command */
    command_file: String,

    /// The Number of commands from the command list that are wrapped into each job
    #[arg(short, long)]
    chunksize: u32,

    /// The number of commands each job runs in parallel. If the chunk size (-c) is smaller than -j then only chunk size commands will run in parallel. This option can also be expressed as a percentage (e.g. 100%%) of the total available cores
    #[arg(short = 'j', long)]
    cores: String,

    /// Create jobfiles but do not submit or run any commands
    #[arg(short = 'n', long)]
    dryrun: bool,

    /// Set job name (defaults to name of command file, or STDIN)
    #[arg(short = 'N', long)]
    jobname: String,

    /// Memory required for each job (e.g. --mem 1G). This value will  be set on each variable specified in --memvars. To not set any memory requirement, set this to 0
    #[arg(short, long)]
    mem: u64,

    /// The name of queue to submit jobs to (defaults to no queue)
    #[arg(short, long)]
    queue: String,

    /// The requested number of processors per job (aka ppn on PBS, slots on SGE, CPUs per task on SLURM). Cores can be over subscribed if -j is larger than --ppj (useful to make use of hyper-threading on some systems)
    #[arg(short, long)]
    ppj: u32,

    /// The maximum walltime for an array job element or individual job
    #[arg(short, long)]
    walltime: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    // ADVANCED OPTIONS
    /// Wait for successful completion of job(s) with name matching given glob pattern or job id matching given job id(s) before starting
    #[arg(long, action = ArgAction::Append, help_heading = "Advanced Options")]
    depend: Option<Vec<String>>,

    /// Job working directory
    #[arg(short = 'd', long, help_heading = "Advanced Options")]
    workdir: Option<PathBuf>,

    /// Directory to save store log files (default={workdir}/logs)
    #[arg(long, help_heading = "Advanced Options")]
    logdir: Option<PathBuf>,

    /** Custom options passed directly to the queuing system (e.g --options "-l vf=8G". This option can be given multiple times" */
    #[arg(short, long, action = ArgAction::Append, help_heading = "Advanced Options")]
    options: Vec<String>,

    /** A line to insert verbatim at the start of the script, and will be run once per job. This option can be given multiple times */
    #[arg(long, action = ArgAction::Append, help_heading = "Advanced Options")]
    header: Vec<String>,

    /** A line to insert verbatim at the end of the script, and will be run once per job. This option can be given multiple times. */
    #[arg(long, action = ArgAction::Append, help_heading = "Advanced Options")]
    footer: Vec<String>,

    /** (PBS and SLURM only) Nodes to request per job */
    #[arg(long, help_heading = "Advanced Options")]
    nodes: u32,

    /** (SGE-only) The parallel environment to use if more than one processor per job is requested */
    #[arg(long, help_heading = "Advanced Options")]
    sge_pe: String,

    /** A comma-separated list of variables to set with the memory limit given by the --mem option (e.g. --memvars=h_vmem,vf)"" */
    #[arg(long, help_heading = "Advanced Options")]
    memvars: String,

    /** (PBS-only) String to be inserted into nodes= line of job */
    #[arg(long, action = ArgAction::Append, help_heading = "Advanced Options")]
    pbs_nodes_spec: Vec<String>,

    /** Submit individual jobs instead of an array job */
    #[arg(short, long, help_heading = "Advanced Options")]
    individual: bool,

    /**
    * The type of queueing system to use. For reference, 'pbs' and 'sge'
    both make calls to qsub to submit jobs; 'slurm' calls sbatch; 'local'
    runs the entire command list (without chunking) locally; 'container'
    creates a joblist and metadata file, to pass commands out of a container
    to a monitoring process for submission to a batch system.
    */
    #[arg(short = 'b', long, help_heading = "Advanced Options")]
    system: String, // System,

    /** Determines how your environment is propagated when your job runs. "copied" records your environment settings in the job submission script, "batch" uses the cluster's mechanism for propagating your environment, and "none" does not propagate any environment variables. */
    #[arg(long, default_value = "copied", help_heading = "Advanced Options")]
    env: String, // Env

    /** Shell to use for spawning jobs and launching single commands  */
    #[arg(long, help_heading = "Advanced Options")]
    shell: String,

    /** For SGE, PBS and SLURM, blocks execution until jobs are finished. */
    #[arg(long, help_heading = "Advanced Options")]
    block: bool,
}

fn main() {
    // let config = config::Config::from_env();
    // println!("{:#?}", config);
    let args = Arguments::parse();
    println!("{:#?}", args);

    let defaults = Config::get_defaults();
}
