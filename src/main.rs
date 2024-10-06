mod problem;

use crate::problem::{Problem, Sample};
use clap::Parser;
use eyre::WrapErr;
use fantoccini::{elements::Element, Client, ClientBuilder, Locator};
use futures::{stream, StreamExt, TryStreamExt};
use itertools::chain;
use markdown::{Block, Span};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// The ZeroJudge crawler.
#[derive(Parser)]
struct Opts {
    /// The URL fo the problem list site.
    #[clap(long, default_value = "https://zerojudge.tw/Problems?tag=APCS")]
    pub url: String,

    /// The output directory to save the scrapped data.
    #[clap(long, default_value = "problems")]
    pub output_dir: PathBuf,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let Opts { url, output_dir } = Opts::parse();

    // Create the output directory
    fs::create_dir(&output_dir)
        .wrap_err_with(|| format!("unabel to create directory {}", output_dir.display()))?;

    // Initialize the crawler
    let client = ClientBuilder::rustls()?
        .connect("http://localhost:4444")
        .await
        .wrap_err("failed to connect to WebDriver")?;

    // Crawl the website and scrape problem data
    let problems = crawl(client, &url).await?;

    // Generate Markdown documents for each problem
    for problem in problems {
        let problem_dir = output_dir.join(&problem.title);
        fs::create_dir(&problem_dir)
            .wrap_err_with(|| format!("unable to create directory '{}'", problem_dir.display()))?;
        save_problem(&problem_dir, problem)?;
    }

    Ok(())
}

async fn crawl(client: Client, url: &str) -> eyre::Result<Vec<Problem>> {
    // Visit the problem list site and obtain links to problems
    let links = {
        client.goto(&url).await?;
        let links = client
            .find_all(Locator::XPath(
                "/html/body/div[3]/div/table/tbody/tr[*]/td[3]/a",
            ))
            .await
            .wrap_err_with(|| format!("unable to locate problem links in page '{url}'"))?;

        let links: Vec<_> = stream::iter(links)
            .map(|link| async move {
                let href = link
                    .prop("href")
                    .await?
                    .expect("the 'href' attribute is not fonud");
                eyre::Ok(href)
            })
            .buffered(1)
            .try_collect()
            .await?;

        links
    };

    // Vist each problem page one-by-one
    let problems: Vec<_> = stream::iter(links)
        .map(|link| {
            let client = &client;

            async move {
                client
                    .goto(&link)
                    .await
                    .wrap_err_with(|| format!("unable to visit '{link}'"))?;

                let title = client
                    .find(Locator::Id("problem_title"))
                    .await?
                    .text()
                    .await?;
                let content = {
                    let elem = client.find(Locator::Id("problem_content")).await?;
                    element_to_markdown(&elem).await?
                };

                let input_desc = {
                    let elem = client.find(Locator::Id("problem_theinput")).await?;
                    element_to_markdown(&elem).await?
                };
                let output_desc = {
                    let elem = client.find(Locator::Id("problem_theoutput")).await?;
                    element_to_markdown(&elem).await?
                };

                let samples: Vec<_> = {
                    let rows = client
                        .find_all(Locator::XPath(
                            "/html/body/div[3]/div[2]/div/div[2]/div[1]/div[position() >= 2]",
                        ))
                        .await?;

                    futures::stream::iter(rows)
                        .map(|row| async move {
                            let input = row
                                .find(Locator::XPath("div[1]/div/div[2]/pre"))
                                .await?
                                .text()
                                .await?;
                            let output = row
                                .find(Locator::XPath("div[2]/div/div[2]/div/pre"))
                                .await?
                                .text()
                                .await?;
                            eyre::Ok(Sample { input, output })
                        })
                        .buffered(1)
                        .try_collect()
                        .await?
                };

                let hint = client
                    .find(Locator::Id("problem_hint"))
                    .await?
                    .html(true)
                    .await?;

                let problem = Problem {
                    title,
                    content,
                    input_desc,
                    output_desc,
                    hint,
                    samples,
                };

                eyre::Ok(problem)
            }
        })
        .buffered(1)
        .try_collect()
        .await?;

    Ok(problems)
}

/// Transform an HTML element to Markdown blocks.
async fn element_to_markdown(root: &Element) -> eyre::Result<Vec<Block>> {
    let html = root.html(true).await?;
    let markdown = html2md::parse_html(&html);
    let blocks = markdown::tokenize(&markdown);
    Ok(blocks)
}

/// Save the problem content to a specified directory.
fn save_problem(problem_dir: &Path, problem: Problem) -> eyre::Result<()> {
    let Problem {
        title,
        content,
        input_desc,
        output_desc,
        hint,
        samples,
    } = problem;

    let make_title = |title| Block::Header(vec![Span::Text(title)], 2);

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
        [make_title("Task Description".to_string())],
        content,
        [make_title("Input Format".to_string())],
        input_desc,
        [make_title("Output Format".to_string())],
        output_desc,
        samples_blocks,
    )
    .collect();

    let problem_path = problem_dir.join("problem.md");
    let markdown_data = markdown::generate_markdown(blocks);
    fs::write(&problem_path, markdown_data)
        .wrap_err_with(|| format!("unable to create file '{}'", problem_path.display()))?;

    Ok(())
}
