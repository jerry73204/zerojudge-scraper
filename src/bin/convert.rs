use clap::Parser;
use eyre::{bail, WrapErr};
use itertools::chain;
use itertools::Itertools;
use markdown::{Block, Span};
use rayon::prelude::*;
use scraper::{Html, Selector};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// The ZeroJudge page to JudgeGirl page converter.
#[derive(Parser)]
struct Opts {
    /// The output directory to save the scrapped data.
    #[clap(long, default_value = "problems")]
    pub data_dir: PathBuf,
}

fn main() -> eyre::Result<()> {
    let Opts { data_dir } = Opts::parse();

    let dirs: Vec<_> = data_dir
        .read_dir()?
        .filter_map(|entry| {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("{err}");
                    return None;
                }
            };

            let path = entry.path();
            if !path.is_dir() {
                eprintln!("Skip {} because it's not a directory", path.display());
                return None;
            }

            Some(path)
        })
        .collect();

    dirs.into_par_iter().for_each(|dir| {
        if let Err(err) = convert_problem(&dir) {
            eprintln!("Unable to convert problem at '{}': {err}", dir.display());
        }
    });

    Ok(())
}

/// Transform an HTML element to Markdown blocks.
fn html_to_markdown(html: &str) -> eyre::Result<Vec<Block>> {
    let markdown = html2md::parse_html(html);
    let blocks = markdown::tokenize(&markdown);
    Ok(blocks)
}

/// Save the problem content to a specified directory.
fn convert_problem(problem_dir: &Path) -> eyre::Result<()> {
    let document = {
        let source_path = problem_dir.join("source.html");
        let source = fs::read_to_string(&source_path)
            .wrap_err_with(|| format!("unable to read {}", source_path.display()))?;
        Html::parse_document(&source)
    };

    let select_first = |path: &str| {
        let Some(elem) = document.select(&Selector::parse(path).unwrap()).next() else {
            bail!("unable to locate '{path}' element");
        };
        eyre::Ok(elem)
    };

    let title_elem = select_first("#problem_title")?;
    let content_elem = select_first("#problem_content")?;
    let input_desc_elem = select_first("#problem_theinput")?;
    let output_desc_elem = select_first("#problem_theoutput")?;
    let hint_elem = select_first("#problem_hint")?;

    let title = title_elem.text().next().unwrap();
    let hint = html_to_markdown(&hint_elem.inner_html())?;
    let content = html_to_markdown(&content_elem.inner_html())?;
    let input_desc = html_to_markdown(&input_desc_elem.inner_html())?;
    let output_desc = html_to_markdown(&output_desc_elem.inner_html())?;

    let selector = Selector::parse(
        "html \
             body \
             div:nth-child(3) \
             div:nth-child(2) \
             div \
             div:nth-child(2) \
             div:nth-child(1) \
             div:nth-child(n + 2)",
    )
    .unwrap();
    let rows = document.select(&selector);
    let samples: Vec<_> = rows
        .map(|row| {
            let select_text = |path| {
                let Some(elem) = row.select(&Selector::parse(path).unwrap()).next() else {
                    bail!("unable to locate '{path}' element");
                };
                let Some(text) = elem.text().next() else {
                    bail!("no text found");
                };
                eyre::Ok(text)
            };
            let input = select_text("div:nth-child(1) div div:nth-child(2) pre")?.to_string();
            let output = select_text("div:nth-child(2) div div:nth-child(2) pre")?.to_string();
            eyre::Ok(Sample { input, output })
        })
        .try_collect()?;

    let make_title = |title| Block::Header(vec![Span::Text(title)], 2);

    // Save input/output examples
    for (sample, nth) in samples.iter().zip(1..) {
        let Sample { input, output } = sample;
        let intput_file = problem_dir.join(format!("input.{nth}"));
        let output_file = problem_dir.join(format!("output.{nth}"));
        fs::write(&intput_file, input)?;
        fs::write(&output_file, output)?;
    }

    let samples_blocks = if samples.len() == 1 {
        let Sample { input, output } = samples.into_iter().next().unwrap();
        vec![
            make_title("Sample Input".to_string()),
            Block::CodeBlock(None, input.to_string()),
            make_title("Sample Output".to_string()),
            Block::CodeBlock(None, output.to_string()),
        ]
    } else {
        samples
            .into_iter()
            .zip(1..)
            .flat_map(|(sample, nth)| {
                let Sample { input, output } = sample;
                [
                    make_title(format!("Sample Input {nth}")),
                    Block::CodeBlock(None, input.to_string()),
                    make_title(format!("Sample Output {nth}")),
                    Block::CodeBlock(None, output.to_string()),
                ]
            })
            .collect()
    };

    let blocks: Vec<Block> = chain!(
        [make_title(title.to_string())],
        [make_title("Task Description".to_string())],
        content,
        [make_title("Input Format".to_string())],
        input_desc,
        [make_title("Output Format".to_string())],
        output_desc,
        samples_blocks,
        [make_title("Hint".to_string())],
        hint,
    )
    .collect();

    let problem_path = problem_dir.join("problem.md");
    let markdown_data = markdown::generate_markdown(blocks);
    fs::write(&problem_path, markdown_data)
        .wrap_err_with(|| format!("unable to create file '{}'", problem_path.display()))?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub title: String,
    pub content: Vec<Block>,
    pub input_desc: Vec<Block>,
    pub output_desc: Vec<Block>,
    pub hint: String,
    pub samples: Vec<Sample>,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub input: String,
    pub output: String,
}
