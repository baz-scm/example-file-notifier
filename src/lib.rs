use notify_debouncer_mini::DebouncedEvent;
use std::ffi::OsStr;
const BLOCK_LISTED_DIRS: &'static [&str] = &[
    "node_modules",
    "venv",
    "target",
];



pub fn should_notify(event: &DebouncedEvent) -> bool {
    let event_path = event.path.as_path();
    let should_notify = event_path.iter().fold(true, |acc: bool, part: &OsStr| {
        let part_str = part.to_str().unwrap_or("");
        acc && !part_str.is_empty() && !part_str.starts_with(".") && !BLOCK_LISTED_DIRS.contains(&part_str)
    });

    should_notify
}
