# TouchTyping learning app

Application to teach TouchTyping.

## Project aim

Project aim is:

   * should inlcude visualisation of easy customizable keyboard
   * training content should be possible to adapt to many languages
   * training should include all steps from very begining of blind typing
   * training should be done step by step while introducing letters one by one

## Statuss

Project is in active development phase and lot of code is subject to change.

## Project roadmap

Project is far from stable. For stable version we should implement:

  * basic keyboard graphical representation ✅︎
  * lesson configuration commands using yaml data files
    - show key location ✅︎
    - show explanation text ✅︎
    - entry training with and without backspace usage - partly
    - speed improvement exercises with speed measurement
  * full course on query keyboard in yaml files
  * lesson table of contents, to choose any lesson to work on - partly
  * save state for lessons ✅︎
  * version packaging

## Known issues

  * space key press is not recognized
  * key bacground is not updated on key press

## Run project

To run project you should have rust infrastructure set up.
Then you can compile and run project using commend:

```
cargo run
```

## Similar projects

https://www.typingstudy.com/
