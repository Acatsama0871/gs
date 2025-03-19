use crate::modules::response_models::utils::get_first_n_page;
use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub fn show_func(pages: u8, all: bool) -> Result<()> {
    let response = get_first_n_page(pages, all)?;

    let response_json = serde_json::to_string_pretty(&response)?;
    let mut file = File::create("test_data.json")?;
    file.write_all(response_json.as_bytes())?;

    Ok(())
}
