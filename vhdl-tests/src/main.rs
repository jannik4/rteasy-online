use anyhow::{anyhow, bail, ensure, Context, Error, Result};
use compiler_backend_vhdl::{Args, BackendVhdl};
use std::{
    env, fs,
    path::PathBuf,
    process::{self, Command},
};

const VHDL_STANDARD: &str = "08";

fn main() {
    let test_results = match run() {
        Ok(test_results) => test_results,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };

    let mut passed = 0;
    let mut failed = Vec::new();
    for test_result in test_results {
        match test_result {
            Ok(_tb) => passed += 1,
            Err(e) => failed.push(e),
        }
    }

    if failed.len() == 0 {
        println!("PASSED\n\n{} passed; {} failed", passed, failed.len());
    } else {
        for (tb, err) in &failed {
            eprintln!("Test {} failed\n\n{:?}", tb.name, err);
            eprintln!("\n--------------------------------\n\n");
        }

        eprintln!("FAILED\n\n{} passed; {} failed", passed, failed.len());

        process::exit(1);
    }
}

fn run() -> Result<Vec<Result<Tb, (Tb, Error)>>> {
    let mut test_results = Vec::new();

    for tb in testbenches().context("failed to load testbenches")? {
        let result = tb.run();
        if tb.name.starts_with("fail") {
            match result {
                Ok(()) => {
                    test_results.push(Err((tb, anyhow!("executed successfully, expected error"))))
                }
                Err(TbError::Prepare(e)) => test_results.push(Err((tb, e))),
                Err(TbError::Run(_e)) => test_results.push(Ok(tb)),
            }
        } else {
            match result {
                Ok(()) => test_results.push(Ok(tb)),
                Err(TbError::Prepare(e)) => test_results.push(Err((tb, e))),
                Err(TbError::Run(e)) => test_results.push(Err((tb, e))),
            }
        }
    }

    Ok(test_results)
}

fn testbenches() -> Result<Vec<Tb>> {
    let current_dir = env::current_dir()?;
    let testbenches_dir = current_dir.join("testbenches/src");
    ensure!(
        fs::metadata(&testbenches_dir).is_ok(),
        "testbenches directory not found. make sure to run tests from the correct working directory"
    );

    let mut testbenches = Vec::new();

    for entry in fs::read_dir(&testbenches_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_dir() {
            let name = path
                .file_name()
                .context("expected file name")?
                .to_str()
                .context("directory name utf8 error")?
                .to_owned();
            let target_dir = testbenches_dir.join(format!("../target/{}", name));

            testbenches.push(Tb { name, src_dir: path, target_dir });
        }
    }

    Ok(testbenches)
}

#[derive(Debug)]
struct Tb {
    name: String,
    src_dir: PathBuf,
    target_dir: PathBuf,
}

#[derive(Debug)]
enum TbError {
    Prepare(Error),
    Run(Error),
}

impl Tb {
    fn run(&self) -> Result<(), TbError> {
        // Prepare
        (|| {
            // Check for files
            ensure!(
                self.src_dir.join(self.rt_file_name()).is_file()
                    && self.src_dir.join(self.tb_file_name()).is_file(),
                "could not find {} and {} in {:?}",
                self.rt_file_name(),
                self.tb_file_name(),
                self.src_dir
            );

            // Create target dir
            fs::create_dir_all(&self.target_dir)?;

            // Compile and save rt code to vhdl
            self.compile_and_save().context("failed to compile rt code")?;

            // Copy testbench to target
            fs::copy(
                self.src_dir.join(self.tb_file_name()),
                self.target_dir.join(self.tb_file_name()),
            )?;

            // Analyze
            self.run_cmd(&format!("ghdl -a --std={} {}", VHDL_STANDARD, self.vhdl_file_name()))?;
            self.run_cmd(&format!("ghdl -a --std={} {}", VHDL_STANDARD, self.tb_file_name()))?;

            // Elaborate
            self.run_cmd(&format!("ghdl -e --std={} {}", VHDL_STANDARD, self.tb_name()))?;
            Ok(())
        })()
        .map_err(TbError::Prepare)?;

        // Run
        self.run_cmd(&format!(
            "ghdl -r --std={} {} --assert-level=error --wave={}",
            VHDL_STANDARD,
            self.tb_name(),
            self.ghw_file_name()
        ))
        .map_err(TbError::Run)?;

        Ok(())
    }

    fn compile_and_save(&self) -> Result<()> {
        let rt_code = fs::read_to_string(self.src_dir.join(self.rt_file_name()))?;

        let ast = match parser::parse(&rt_code) {
            Ok(ast) => ast,
            Err(e) => bail!("{}", parser::pretty_print_error(&e, &rt_code, None, false)),
        };

        let vhdl = match compiler::compile(
            &BackendVhdl,
            Args { module_name: self.name.clone() },
            ast,
            &Default::default(),
        ) {
            Ok(vhdl) => vhdl,
            Err(e) => bail!("{}", e.pretty_print(&rt_code, None, false)),
        };

        fs::write(self.target_dir.join(self.vhdl_file_name()), vhdl.render()?)?;

        Ok(())
    }

    fn run_cmd(&self, command: &str) -> Result<()> {
        let program = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
        let exec_arg = if cfg!(target_os = "windows") { "/C" } else { "-c" };

        let output = Command::new(program)
            .args([exec_arg, command])
            .current_dir(&self.target_dir)
            .output()
            .with_context(|| format!("failed to execute cmd `{}`", command))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!(
                "failed to execute cmd `{}`\n\nstdout:\n{}\nstderr:\n{}",
                command,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            ))
        }
    }

    fn tb_name(&self) -> String {
        format!("{}_tb", self.name)
    }

    fn rt_file_name(&self) -> String {
        format!("{}.rt", self.name)
    }
    fn tb_file_name(&self) -> String {
        format!("{}_tb.vhdl", self.name)
    }
    fn vhdl_file_name(&self) -> String {
        format!("{}.vhdl", self.name)
    }
    fn ghw_file_name(&self) -> String {
        format!("{}.ghw", self.name)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        panic!("use `cargo run` instead");
    }
}
