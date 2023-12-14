use textwrap::dedent;

pub const CONTAINER_TEMPLATE: &str = "";

pub const PBS_HEADER_TEMPLATE: &str = dedent(
    "
    #!{shell}
    #PBS -S {shell}
    #PBS -l nodes={nodes}:{nodes_spec}ppn={ppj}
    #PBS -j oe
    #PBS -o {logdir}
    #PBS -d {workdir}
    #PBS -N {job_name}
    #PBS {o_memopts}
    #PBS {o_queue}
    #PBS {o_array}
    #PBS {o_walltime}
    #PBS {o_dependencies}
    #PBS {o_options}
    #PBS {o_env}
    #PBS {o_block}
    {env}
    {header_commands}
    ARRAY_IND=$PBS_ARRAYID
",
);

pub const SGE_HEADER_TEMPLATE: &str = dedent(
    "
    #!{shell}
    #$ -S {shell}
    #$ {ppj}
    #$ -j y
    #$ -o {logdir}
    #$ -wd {workdir}
    #$ -N {job_name}
    #$ {o_memopts}
    #$ {o_queue}
    #$ {o_array}
    #$ {o_walltime}
    #$ {o_dependencies}
    #$ {o_options}
    #$ {o_env}
    #$ {o_block}
    {env}
    {header_commands}
    ARRAY_IND=$SGE_TASK_ID
",
);

pub const SLURM_HEADER_TEMPLATE: &str = dedent(
    "
    #!{shell}
    #SBATCH --nodes={nodes}
    #SBATCH {ppj}
    #SBATCH {logfile}
    #SBATCH -D {workdir}
    #SBATCH --job-name={job_name}
    #SBATCH {o_memopts}
    #SBATCH {o_queue}
    #SBATCH {o_array}
    #SBATCH {o_walltime}
    #SBATCH {o_dependencies}
    #SBATCH {o_options}
    #SBATCH {o_env}
    #SBATCH {o_block}
    {env}
    {header_commands}
    ARRAY_IND=$SLURM_ARRAY_TASK_ID
",
);

pub const LOCAL_TEMPLATE: &str = "
    #!{shell}
    {env}
    {header_commands}
    cd {workdir}
";
