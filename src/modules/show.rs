use crate::modules::utils;
use anyhow::Result;
use colored::Colorize;
use cli_table::{print_stdout, Cell, Style, Table, format::Justify};
//use duckdb::Connection;
//use std::path::PathBuf;

pub fn show_func() -> Result<()> {
    // connect to db
    // let db_path: PathBuf = [".", "data", "database.db"].iter().collect();
    // let db_conn = Connection::open(db_path)?;
    // db_conn.execute(
    //     "CREATE TABLE IF NOT EXISTS citation (time TIMESTAMP, citations INTEGER)",
    //     [],
    // )?;
    // db_conn.execute(
    //     "CREATE TABLE IF NOT EXISTS paper (time TIMESTAMP, title TEXT, citations INTEGER)",
    //     [],
    // )?;

    // send request
    let author_page_info = utils::get_author_page()?;
    let citation_number = utils::get_citation_num_from_author_info(&author_page_info)?;
    let paper_list = utils::get_paper_list_from_author_info(&author_page_info)?;

    // print as tables
    let mut paper_list_cells = vec![
        vec!["Total".green().cell().bold(true),
        format!("{}", citation_number).green().cell().bold(true)]
    ];
    for cur_paper in paper_list {
        paper_list_cells.push(vec![
            cur_paper.title.cell().bold(true),
            cur_paper.num_citations.cell().bold(true),
        ]);
    }
    let paper_list_table = paper_list_cells.table().title(vec![
        "Item".blue().cell().bold(true).justify(Justify::Center),
        "Citations".blue().cell().bold(true),
    ]);
    print_stdout(paper_list_table)?;

    Ok(())
}
