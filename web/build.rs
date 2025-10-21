fn main() {
    // check folder 'node_modules' exists or not'
    if !std::path::Path::new("node_modules").exists() {
        // run npm install
        println!("cargo:info=Running npm install...");
        std::process::Command::new("pnpm")
            .arg("run")
            .arg("install")
            .status()
            .expect("Failed to execute pnpm install");
    }

    // run pnpm build
    println!("cargo:info=Running pnpm build...");
    std::process::Command::new("pnpm")
        .arg("run")
        .arg("build")
        .status()
        .expect("Failed to execute pnpm install");
}
