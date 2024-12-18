<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2024

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `49.0µs` | `49.7µs` |
| [Day 2](./src/bin/02.rs) | `99.1µs` | `128.5µs` |
| [Day 3](./src/bin/03.rs) | `101.5µs` | `142.1µs` |
| [Day 4](./src/bin/04.rs) | `296.7µs` | `46.7µs` |
| [Day 5](./src/bin/05.rs) | `241.4µs` | `384.6µs` |
| [Day 6](./src/bin/06.rs) | `232.4µs` | `27.1ms` |
| [Day 7](./src/bin/07.rs) | `183.0µs` | `198.4µs` |
| [Day 8](./src/bin/08.rs) | `18.4µs` | `20.8µs` |
| [Day 9](./src/bin/09.rs) | `81.7µs` | `871.9µs` |
| [Day 10](./src/bin/10.rs) | `199.0µs` | `22.9µs` |
| [Day 11](./src/bin/11.rs) | `218.6µs` | `8.2ms` |
| [Day 12](./src/bin/12.rs) | `194.2µs` | `309.6µs` |
| [Day 13](./src/bin/13.rs) | `72.3µs` | `72.3µs` |
| [Day 14](./src/bin/14.rs) | `76.4µs` | `70.6ms` |
| [Day 15](./src/bin/15.rs) | `142.9µs` | `1.5ms` |
| [Day 16](./src/bin/16.rs) | `857.0µs` | `12.3ms` |
| [Day 17](./src/bin/17.rs) | `821.0ns` | `273.6ms` |

**Total: 398.61ms**
<!--- benchmarking table --->

✨ You can start solving puzzles now! Head to the [Usage section](#usage) to see how to use this template. If you like, you can configure [some optional features](#optional-template-features).

## Usage

### ➡️ Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/template.txt) has _tests_ referencing its _example_ file in `./data/examples`. Use these tests to develop and debug your solutions against the example input. In VS Code, `rust-analyzer` will display buttons for running / debugging these unit tests above the unit test blocks.

> [!TIP]
> If a day has multiple example inputs, you can use the `read_file_part()` helper in your tests instead of `read_file()`. If this e.g. applies to day 1, you can create a second example file `01-2.txt` and invoke the helper like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));`. This supports an arbitrary number of example files.

### ➡️ Lint code

```sh
cargo clippy
```

### Automatically track ⭐️ progress in the readme

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

-   `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`.
-   `AOC_YEAR`: the year you want to track. Example: `2021`.
-   `AOC_SESSION`: an active session[^2] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

Go to the _Variables_ tab in your repository settings and create the following variable:

-   `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker. After you complete AoC or no longer work on it, you can set this to `false` to disable the CI.

✨ You can now run this action manually via the _Run workflow_ button on the workflow page. If you want the workflow to run automatically, uncomment the `schedule` section in the `readme-stars.yml` workflow file or add a `push` trigger.

### Enable code formatting / clippy checks in the CI

Uncomment the respective sections in the `ci.yml` workflow.

### Use DHAT to profile heap allocations

If you are not only interested in the runtime of your solution, but also its memory allocation profile, you can use the template's [DHAT](https://valgrind.org/docs/manual/dh-manual.html) integration to analyze it. In order to activate DHAT, call the `solve` command with the `--dhat` flag.

```sh
cargo solve 1 --dhat

# output:
#     Running `target/dhat/1`
# dhat: Total:     276 bytes in 3 blocks
# dhat: At t-gmax: 232 bytes in 2 blocks
# dhat: At t-end:  0 bytes in 0 blocks
# dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
# Part 1: 9001 (4.1ms)
```

The command will output some basic stats to the command-line and generate a `dhat-heap.json` report in the repo root directory.

You can pass the report a tool like [dh-view](https://nnethercote.github.io/dh_view/dh_view.html) to view a detailed breakdown of heap allocations.

### Use VS Code to debug your code

1.  Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
2.  Set breakpoints in your code. [^3]
3.  Click _Debug_ next to the unit test or the _main_ function. [^4]
4.  The debugger will halt your program at the specific line and allow you to inspect the local stack. [^5]
