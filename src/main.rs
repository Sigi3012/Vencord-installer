use std::env;
use std::fs;
use std::io::Cursor;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const CLI_DOWNLOAD_URL: &str = "https://github.com/Vencord/Installer/releases/latest/download/VencordInstallerCli.exe";

async fn install_cli(url: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create("./dist/Installer/VencordInstallerCli.exe")?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

async fn build_vencord() {
     let output = Command::new("docker") 
        .args(["compose", "up"])
        .output()
        .expect("Failed to execute docker compose up");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <vencord_path>", args[0]);
        return;
    }

    let vencord_path = &args[1];

    if let Err(err) = fs::create_dir_all("./dist/Installer") {
        eprintln!("Failed to creat directory (somehow): {}", err);
        return;
    }

    if fs::metadata("./dist/Installer/VencordInstallerCli.exe").is_ok() {
        println!("Injector already installed")
    } else {
        println!("Injector is not installed, installing..");
        match install_cli(CLI_DOWNLOAD_URL.to_string()).await {
            Ok(_) => println!("Successfully downloaded"),
            Err(_) => eprintln!("Something went wrong :(")
        };
    }

    build_vencord().await;

    let status = std::process::Command::new(format!(
        "{}/dist/installer/VencordInstallerCli.exe",
        vencord_path
    ))
    .env("VENCORD_USER_DATA_DIR", vencord_path)
    .env("VENCORD_DEV_INSTALL", "1")
    .status();

    if let Err(e) = status {
        eprintln!("Something went wrong: {}", e);
    }
}
