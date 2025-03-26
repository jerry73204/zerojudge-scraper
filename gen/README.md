# The Testcase Generator for ZeroJudge Problems

## Installation

- Install Rust toolchain. You can visit [rust.rs](https://rustup.rs/) to
  follow the instructions.

- Prepare the working repositroy.

  ```sh
  git clone https://github.com/jerry73204/zerojudge-scraper.git
  cd zerojudge-scraper/gen
  ```

## Usage

Edit the `e288.json` configure the testcase generation parameters for
problem e288 for example.

```sh
nano config/e288.json5
```


Run this command generate testcases for problem E288 for example. The
generated testcases can be found at e288 directory.

```sh
./run.sh e288
```
