# ZeroJudge Scraper

The ZeroJudge problem set scraper downloads problem pages and
transforms them to JudgeGirl format.

This repository provides the scraper problem. The scrapped results are
saved in the `problems` branch.

## Prerequisite

- Rust toolchain

    Please visit [rustup.rs](https://rustup.rs/) and follow the
    instructions to install the toolchain.

## Usage

Download the repository.

```sh
git clone https://github.com/jerry73204/zerojudge-scraper.git
cd zerojudge-scraper
```

Run the scraper problem. The program will be built automatically.

```sh
cargo run --release
```

After the scraping finishes, the problem set is saved in the problems
directory.

```sh
cd problems/
```

## License

The licensing of the project is not decided yet.
