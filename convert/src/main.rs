use clap::Parser;
use eyre::{bail, WrapErr};
use itertools::Itertools;
use markdown::{Block, ListItem, Span};
use rayon::prelude::*;
use scraper::{Html, Selector};
use std::{
    fs,
    path::{Path, PathBuf},
};
use url::Url;

/// The ZeroJudge page to JudgeGirl page converter.
#[derive(Parser)]
struct Opts {
    /// The output directory to save the scrapped data.
    #[clap(long, default_value = "data")]
    pub data_dir: PathBuf,

    #[clap(long, default_value = "zerojudge.tw")]
    pub host_name: String,
}

fn main() -> eyre::Result<()> {
    let Opts {
        data_dir,
        host_name,
    } = Opts::parse();

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

    dirs.into_par_iter().for_each(|problem_dir| {
        if let Err(err) = convert_problem(&problem_dir, &host_name) {
            eprintln!(
                "Unable to convert problem at '{}': {err}",
                problem_dir.display()
            );
        }
    });

    Ok(())
}

/// Transform an HTML element to Markdown blocks.
fn html_to_markdown(html: &str) -> eyre::Result<String> {
    // Convert the snipplet to Markdown
    let markdown = html2md::parse_html(html);

    // Transform the Markdown data
    fn convert_block(block: &mut Block) {
        match block {
            Block::Header(spans, _) => spans.iter_mut().for_each(convert_span),
            Block::Paragraph(spans) => spans.iter_mut().for_each(convert_span),
            Block::Blockquote(blocks) => blocks.iter_mut().for_each(convert_block),
            Block::CodeBlock(_, _) => {}
            Block::OrderedList(items, _) => items.iter_mut().for_each(convert_list_item),
            Block::UnorderedList(items) => items.iter_mut().for_each(convert_list_item),
            Block::Raw(raw) => convert_raw(raw),
            Block::Hr => {}
        }
    }

    fn convert_span(span: &mut Span) {
        match span {
            Span::Break => {}
            Span::Text(_) => {}
            Span::Code(_) => {}
            Span::Link(_text, link, _) => {}
            Span::Image(_text, link, _) => {}
            Span::Emphasis(spans) => spans.iter_mut().for_each(convert_span),
            Span::Strong(spans) => spans.iter_mut().for_each(convert_span),
        }
    }

    fn convert_list_item(item: &mut ListItem) {
        match item {
            ListItem::Simple(spans) => spans.iter_mut().for_each(convert_span),
            ListItem::Paragraph(blocks) => blocks.iter_mut().for_each(convert_block),
        }
    }

    fn convert_raw(raw: &mut String) {}

    let mut blocks = markdown::tokenize(&markdown);
    blocks.iter_mut().for_each(convert_block);

    Ok(markdown)
}

/// Save the problem content to a specified directory.
fn convert_problem(problem_dir: &Path, host_name: &str) -> eyre::Result<()> {
    let Some(problem_id) = problem_dir.file_name() else {
        bail!("unable to get the file name of '{}'", problem_dir.display());
    };
    let Some(problem_id) = problem_id.to_str() else {
        bail!(
            "'{}' does not have a proper file name",
            problem_dir.display()
        );
    };

    let webpage_dir = problem_dir.join(host_name);
    let webpage_path = problem_dir
        .join(host_name)
        .join(format!("ShowProblem_problemid_{problem_id}.html"));

    let document = {
        let source = fs::read_to_string(&webpage_path)
            .wrap_err_with(|| format!("unable to read {}", webpage_path.display()))?;
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
         > body \
         > div:nth-of-type(3) \
         > div:nth-of-type(2) \
         > div \
         > div:nth-of-type(2) \
         > div:nth-of-type(1) \
         > div:nth-of-type(n + 2)",
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
            let input =
                select_text("div:nth-of-type(1) > div > div:nth-of-type(2) > pre")?.to_string();
            let output = select_text("div:nth-of-type(2) > div > div:nth-of-type(2) > div > pre")?
                .to_string();
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
        format!(
            "# Sample Input\n\
             ```\n\
             {input}\n\
             ```\n\
             # Sample Output\n\
             ```\n\
             {output}\n\
             ```\n"
        )
    } else {
        samples
            .into_iter()
            .zip(1..)
            .map(|(sample, nth)| {
                let Sample { input, output } = sample;
                format!(
                    "# Sample Input {nth}\n\
                     ```\n\
                     {input}\n\
                     ```\n\
                     # Sample Output {nth}\n\
                     ```\n\
                     {output}\n
                     ```\n"
                )
            })
            .join("")
    };

    let markdown_doc = format!(
        "# Task Description\n\
         {content}\n\
         # Input Format\n\
         {input_desc}\n\
         # Output Format\n\
         {output_desc}\n\
         # Hint\n\
         {hint}\n\
         {samples_blocks}\n\
         "
    );

    let markdown_path = webpage_dir.join("problem.md");
    fs::write(&markdown_path, markdown_doc)
        .wrap_err_with(|| format!("unable to create file '{}'", markdown_path.display()))?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub title: String,
    pub content: String,
    pub input_desc: String,
    pub output_desc: String,
    pub hint: String,
    pub samples: Vec<Sample>,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub input: String,
    pub output: String,
}
