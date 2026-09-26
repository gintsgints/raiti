<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

![build](https://github.com/gintsgints/raiti/actions/workflows/rust.yml/badge.svg)

# TouchTyping learning app

Application to teach TouchTyping.

Letter introduction at keyboard:
![screenshot](doc/img/new_letter.png)

Exercise with screen keyboard:
![screenshot](doc/img/exercise.png)

## Project aim

Project aim is:

   * should inlcude visualisation of easy customizable keyboard
   * training content should be possible to adapt to many languages
   * training should include all steps from very begining of blind typing
   * training should be done step by step while introducing letters one by one

## Installation

Download the binary for your platform from the latest
[release](https://github.com/gintsgints/raiti/releases). Lessons are built into
it, so there is nothing else to install.

On macOS and Linux a downloaded file is not executable yet:

```
chmod +x raiti-aarch64-apple-darwin
```

## Custom lessons

Raiti reads its own lessons unless it finds a `data` directory, which it looks
for in this order:

  1. next to `config.yaml`, which is `~/Library/Application Support/raiti` on
     macOS, `~/.config/raiti` on Linux and `%APPDATA%\raiti` on Windows
  2. next to the executable, where earlier releases kept it

The first directory that can be used replaces the built-in lessons completely,
so copy the whole [data](data) directory before editing it rather than dropping
single files in.

A directory is used only if `index.yaml`, the keyboard layout named in
`config.yaml` and every lesson listed in the index are present and readable. If
something is missing, Raiti says so on startup and carries on with the lessons
it ships with.

## Statuss

Project is in active development phase and lot of code is subject to change.

## Project roadmap

Project is far from stable. For stable version we should implement:

  * basic keyboard graphical representation ✅︎
  * lesson configuration commands using yaml data files
    - show ilustrations on correct finger & body positions.
    - show key location ✅︎
    - show explanation text ✅︎
    - pictures of correct sitting and finger positions while typing ✅︎
    - one line exercise with and without Enter at end
    - entry training with and without backspace usage - partly
    - speed improvement exercises with speed measurement
  * full course on query keyboard in yaml files
  * lesson table of contents, to choose any lesson to work on - partly
  * save state for lessons ✅︎
  * version packaging ✅︎

## Known issues

  * space key press is not recognized
  * while loading unfinished lesson at start (after exit), exercises not filled.

## Run project

To run project you should have rust infrastructure set up.
Then you can compile and run project using commend:

```
cargo run
```

## Similar projects

https://www.typingstudy.com/
