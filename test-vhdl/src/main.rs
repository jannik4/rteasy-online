use anyhow::{anyhow, bail, ensure, Context, Error, Result};
use compiler_backend_vhdl::{Args, BackendVhdl};
use std::{
    env, fs,
    path::PathBuf,
    process::{self, Command},
};

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
    let testbenches_dir = current_dir.join("testbenches");
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
            let dir_name = path
                .file_name()
                .context("expected file name")?
                .to_str()
                .context("directory name utf8 error")?;
            let rt_name = format!("{}.rt", dir_name);
            let tb_name = format!("{}_tb.vhdl", dir_name);
            ensure!(
                path.join(&rt_name).is_file() && path.join(&tb_name).is_file(),
                "expected {} and {} in {}",
                rt_name,
                tb_name,
                dir_name
            );

            testbenches.push(Tb {
                name: dir_name.to_owned(),
                rt_code: fs::read_to_string(path.join(&rt_name))?,
                dir: path,
            });
        }
    }

    Ok(testbenches)
}

#[derive(Debug)]
struct Tb {
    name: String,
    rt_code: String,
    dir: PathBuf,
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
            // Compile and save
            self.compile_and_save().context("failed to compile rt code")?;

            // Analyze
            self.run_cmd(&format!("ghdl -a --std=08 {}.gen.vhdl", self.name))?;
            self.run_cmd(&format!("ghdl -a --std=08 {}_tb.vhdl", self.name))?;

            // Elaborate
            self.run_cmd(&format!("ghdl -e --std=08 {}_tb", self.name))?;
            Ok(())
        })()
        .map_err(TbError::Prepare)?;

        // Run
        self.run_cmd(&format!(
            "ghdl -r --std=08 {0}_tb --assert-level=error --wave={0}.ghw",
            self.name
        ))
        .map_err(TbError::Run)?;

        Ok(())
    }

    fn compile_and_save(&self) -> Result<()> {
        let ast = match parser::parse(&self.rt_code) {
            Ok(ast) => ast,
            Err(e) => bail!("{}", parser::pretty_print_error(&e, &self.rt_code, None, false)),
        };

        let vhdl = match compiler::compile(
            &BackendVhdl,
            Args { module_name: self.name.clone() },
            ast,
            &Default::default(),
        ) {
            Ok(vhdl) => vhdl,
            Err(e) => bail!("{}", e.pretty_print(&self.rt_code, None, false)),
        };

        fs::write(self.dir.join(format!("{}.gen.vhdl", self.name)), vhdl.render()?)?;

        Ok(())
    }

    fn run_cmd(&self, command: &str) -> Result<()> {
        let program = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
        let exec_arg = if cfg!(target_os = "windows") { "/C" } else { "-c" };

        let output = Command::new(program)
            .args([exec_arg, command])
            .current_dir(&self.dir)
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        panic!("use `cargo run` instead");
    }
}
