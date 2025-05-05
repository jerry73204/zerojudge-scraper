# ZeroJudge Scraper

The ZeroJudge problem set scraper downloads problem pages and
transforms them to JudgeGirl format.

This repository provides the scraper problem, solutions and testcase
generators.

## Project Layout

All scrapped problem websites and solutions are located in the
[`data`](data) directory. For example,

- [`data/b266/zerojudge.tw/problem.md`](data/b266/zerojudge.tw/problem.md)
  is the problem description converted to Markdown.
- [`data/b266/solution`](data/b266/solution) is the solution in C or
  C++.
- [`data/b266/tests`](data/b266/tests) stores the testcases. It's
  usualy 10 intput/output pairs.

## Generate Testcases

You can find the list of problems that already have generated
testcases in the directory.

- `data/with_tests/`

You can re-generate testcases. To generate testcases for b266 for
exmaple,

```sh
./gen.sh b266
```

## Developer Workflow

### Prepare the Environment

- Rust toolchain

    Please visit [rustup.rs](https://rustup.rs/) and follow the
    instructions to install the toolchain.

## Step 1: Scrap Problem Websites

The step is already done and the scapped files are already in `data`
directory. You can proceed if you want to do it by yourself.

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

## Step 2: Implement


## License

The licensing of the project is not decided yet.
