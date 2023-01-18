use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::{env, path::Path, process::Command, time::Duration};

fn watch(cmd: &str) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(100), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    for events in rx {
        match events {
            Ok(e) => {
                let debounced_event = &e.clone()[0];
                let path = debounced_event.path.to_string_lossy().to_string();

                if path.contains(".rs") && !path.contains("target\\debug") {
                    println!(
                        "[INFO]: Reload with <cargo {}> because <{}> file.",
                        cmd, path
                    );

                    Command::new("cargo")
                        .arg(cmd)
                        .spawn()
                        .expect("hm... this is a problem.");
                }
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];

    watch(cmd).unwrap();
}
