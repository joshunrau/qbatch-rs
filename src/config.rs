use std::env;

/// Environment variables to ignore when copying the environment to the job script
pub const IGNORED_ENV_VARS: [&str; 7] = [
    "ARRAY_IND",
    "BASH_FUNC_*",
    "PBS_ARRAYID",
    "PWD",
    "SGE_TASK_ID",
    "TMP",
    "TMPDIR",
];

/// Returns the environment variable `key`, or `default` if it is not defined,
fn get_env_var(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => String::from(val.trim()),
        Err(_) => {
            println!(
                "Did not find environment variable '{}', using default value: '{}'",
                key, default
            );
            String::from(default)
        }
    }
}

#[derive(Debug)]
pub enum Env {
    Copied,
    Batch,
    None
}

#[derive(Debug)]
pub enum System {
    Container,
    Local,
    Pbs,
    Sge,
    Slurm,
}

impl System {
    pub fn as_str(&self) -> &str {
        match &self {
            Self::Container => "container",
            Self::Local => "local",
            Self::Pbs => "pbs",
            Self::Sge => "sge",
            Self::Slurm => "slurm",
        }
    }
}

#[derive(Debug)]
pub struct Config {
    /// The Number of commands from the command list that are wrapped into each job
    chunksize: u32,
    cores: String,
    mem: u64,
    mem_vars: Vec<String>,
    nodes: u32,
    options: Vec<String>,
    ppj: u32,
    queue: Option<String>,
    script_folder: String,
    sge_pe: String,
    shell: String,
    system: System,
}

impl Config {
    pub fn get_defaults() -> Self {
        let ppj: u32 = get_env_var("QBATCH_PPJ", "1").parse().unwrap();
        Config {
            chunksize: get_env_var("QBATCH_CHUNKSIZE", &ppj.to_string())
                .parse()
                .unwrap(),
            cores: get_env_var("QBATCH_CORES", &ppj.to_string()),
            mem: get_env_var("QBATCH_MEM", "0").parse().unwrap(),
            mem_vars: Vec::from_iter(
                get_env_var("QBATCH_MEMVARS", "mem")
                    .split(",")
                    .map(String::from),
            ),
            nodes: get_env_var("QBATCH_NODES", "1").parse().unwrap(),
            options: Vec::from_iter(
                get_env_var("QBATCH_OPTIONS", "")
                    .split_whitespace()
                    .map(String::from),
            ),
            ppj,
            queue: match &*get_env_var("QBATCH_QUEUE", "") {
                "" => None,
                value => Some(String::from(value)),
            },
            script_folder: get_env_var("QBATCH_SCRIPT_FOLDER", ".qbatch"),
            sge_pe: get_env_var("QBATCH_SGE_PE", "smp"),
            shell: get_env_var("QBATCH_SHELL", "/bin/sh"),
            system: match &*get_env_var("QBATCH_SYSTEM", "local") {
                "local" => System::Local,
                "pbs" => System::Pbs,
                "sge" => System::Sge,
                "slurm" => System::Slurm,
                value => panic!(
                    "Cannot determine system from string '{}', expected one of: {}",
                    value,
                    ["local", "pbs", "sge", "slurm"].join(", ")
                ),
            },
        }
    }
}
