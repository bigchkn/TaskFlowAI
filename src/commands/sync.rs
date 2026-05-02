use crate::storage::Storage;
use crate::roadmap;
use anyhow::Result;
use std::env;

pub fn run<S: Storage>(storage: &S) -> Result<()> {
    let project_root = env::current_dir()?;
    roadmap::generate_roadmaps(storage, &project_root)?;
    println!("Roadmap files synced");
    Ok(())
}
