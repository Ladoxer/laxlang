pub fn watch(path: &str) -> notify::Result<()> {
    use notify::{RecommendedWatcher, RecursiveMode::Recursive, Watcher};

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(tx)?;

    watcher.watch(path.as_ref(), Recursive)?;
    std::thread::spawn(move || {
        for event in rx {
            if let Ok(ev) = event {
                if ev.kind.is_modify() {
                    std::process::Command::new("laxlang")
                        .args(["compile", ev.paths[0].to_str().unwrap()])
                        .spawn()
                        .ok();
                }
            }
        }
    });
    Ok(())
}
