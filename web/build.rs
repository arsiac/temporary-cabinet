use std::path::Path;
use std::process::Command;

fn main() {
    watch_sources();
    check_pnpm();
    install_node_modules();
    build();
}

fn watch_sources() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=package.json");
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=index.html");
    watch_folder("src");
    watch_folder("public");
}

fn watch_folder(dir: &str) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                println!("cargo::rerun-if-changed={}", path.display());
            } else if path.is_dir() {
                watch_folder(path.to_str().unwrap());
            }
        }
    }
}

fn check_pnpm() {
    if Command::new("which").arg("pnpm").output().is_err() {
        eprintln!("cargo:error=pnpm not found in PATH");
        std::process::exit(1);
    }
}

fn install_node_modules() {
    if Path::new("node_modules").exists() {
        return;
    }
    // run npm install
    println!("cargo:info=node_modules not found, running npm install...");
    let status = Command::new("pnpm").arg("install").status();
    match status {
        Ok(status) => {
            if status.success() {
                println!("cargo:info=npm install completed successfully.");
            } else {
                eprintln!("cargo:error=npm install failed.");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("cargo:error=Failed to execute pnpm install: {}", e);
            std::process::exit(1);
        }
    }
}

fn build() {
    println!("cargo:info=Running pnpm build...");
    let status = Command::new("pnpm").arg("run").arg("build").status();

    match status {
        Ok(status) => {
            if status.success() {
                println!("cargo:info=pnpm build completed successfully.");
            } else {
                eprintln!("cargo:error=pnpm build failed.");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("cargo:error=Failed to execute pnpm build: {}", e);
            std::process::exit(1);
        }
    }
}
