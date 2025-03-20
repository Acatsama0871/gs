use crate::modules::response_models::{
    author_page::GoogleScholarResponse, utils::get_n_author_pages,
};
use anyhow::Context;
use anyhow::Result;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table, TableStruct};
use colored::Colorize;
use std::env;
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    #[value(name = "cli-table")]
    CliTable,
    Json,
}

pub async fn show_func(pages: i16, suppress_author: bool, find_author: Option<String>, output_format: OutputFormat) -> Result<()> {
    let response = match find_author {
        None => {
            let author_id = env::var("GOOGLE_SCHOLAR_ID")
                .context("Can not found GOOGLE_SCHOLAR_ID env variable.")?;
            get_n_author_pages(pages, &author_id).await?
        }
        Some(_author_id) => todo!("Finding author is not yet implemented."), // TODO: support the author finding later
    };

    if !suppress_author {
        let author_info = author_level_info_table(&response);
        print_stdout(author_info)?;
    }

    // when pages == 0, suppress all article output
    if pages == 0 {
        return Ok(());
    }

    let article_info = article_info_table(&response);
    print_stdout(article_info)?;

    Ok(())
}

fn author_level_info_table(response: &GoogleScholarResponse) -> TableStruct {
    let author_info = &response.author;
    let author_level_info_cells = vec![
        vec![
            "Name".green().cell().bold(true),
            author_info.name.clone().cell(),
        ],
        vec![
            "Author Id".green().cell().bold(true),
            author_info.author_id.clone().cell(),
        ],
        vec![
            "Citations".green().cell().bold(true),
            author_info.citations.cell(),
        ],
        vec![
            "h-index".green().cell().bold(true),
            author_info.h_index.cell(),
        ],
    ];

    author_level_info_cells.table()
}

fn article_info_table(response: &GoogleScholarResponse) -> TableStruct {
    let articles = &response.articles;

    let mut article_info_cells = vec![];

    for cur_article in articles {
        match &cur_article.cited_by {
            Some(cur_cite_by) => article_info_cells.push(vec![
                cur_article.title.green().cell().bold(true),
                cur_article.year.cell().bold(false),
                cur_cite_by.citations.unwrap_or(0).cell().bold(false),
            ]),
            None => {
                continue;
            }
        }
    }

    article_info_cells.table().title(vec![
        "Title".blue().cell().bold(true).justify(Justify::Center),
        "Year".blue().cell().bold(true).justify(Justify::Center),
        "Citations"
            .blue()
            .cell()
            .bold(true)
            .justify(Justify::Center),
    ])
}
