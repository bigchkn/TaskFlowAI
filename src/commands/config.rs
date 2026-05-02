use crate::storage::Storage;
use anyhow::Result;

pub fn run<S: Storage>(storage: &S, key: String, value: Option<String>) -> Result<()> {
    let mut project = storage.load_project()?;

    if let Some(val) = value {
        // Set config
        project.config.insert(key.clone(), val.clone());
        storage.save_project(&project)?;
        println!("Config set: {} = {}", key, val);
    } else {
        // Get config
        if let Some(val) = project.config.get(&key) {
            println!("{}", val);
        } else {
            return Err(anyhow::anyhow!("Config key '{}' not found", key));
        }
    }

    Ok(())
}
