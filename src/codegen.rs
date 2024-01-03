use log::{debug, error, info, warn};
use std::fs;
use std::io::Write;
use std::time::Instant;

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn generate(&self) {
        let now = Instant::now();

        match fs::create_dir_all("./build") {
            Ok(_) => {}
            Err(err) => {
                error!("failed to create build directory /build {}.", err);
                panic!("failed to create build directory /build {}.", err);
            }
        }

        let mut file = match fs::File::create("./build/build.exe") {
            Err(err) => {
                error!("failed to create executable {}.", err);
                panic!("failed to create executable {}.", err);
            }
            Ok(file) => file,
        };

        match file.write_all("hello, world".as_bytes()) {
            Err(err) => {
                error!("failed to write to executable {}.", err);
                panic!("failed to write to executable {}.", err);
            }
            Ok(file) => {}
        }

        let elapsed = now.elapsed();
        debug!(
            "codegen time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
    }
}
