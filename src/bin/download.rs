use clap::Parser;
use eyre::{bail, WrapErr};
use fantoccini::{Client, ClientBuilder, Locator};
use futures::{stream, StreamExt, TryStreamExt};
use std::{
    borrow::Borrow,
    fs,
    path::{Path, PathBuf},
};
use url::Url;

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
    for link in links {
        if let Err(err) = visit(&client, &link, &output_dir).await {
            eprintln!("An error occurs when visiting {link}");
            eprintln!("{err}");
        }
    }

    Ok(())
}

async fn visit(client: &Client, link: &str, output_dir: &Path) -> eyre::Result<()> {
    eprintln!("Visiting {link}");

    client
        .goto(link)
        .await
        .wrap_err_with(|| format!("unable to visit '{link}'"))?;

    // Find the problem ID
    let url = Url::parse(link).wrap_err_with(|| format!("unable to parse link {link}"))?;
    let Some((_, id)) = url.query_pairs().find(|(key, _)| key == "problemid") else {
        bail!("unable to find the problem ID for {link}");
    };
    let id: &str = id.borrow();

    // Create the output directory
    let problem_dir = output_dir.join(id);
    fs::create_dir(&problem_dir)
        .wrap_err_with(|| format!("unable to create directory {}", problem_dir.display()))?;

    // Save the HTML source code
    {
        let source = client
            .source()
            .await
            .wrap_err_with(|| format!("unable to download source code from {link}"))?;
        let source_path = problem_dir.join("source.html");
        fs::write(&source_path, source)
            .wrap_err_with(|| format!("unable to save source code to {}", source_path.display()))?;
    }
    Ok(())
}
