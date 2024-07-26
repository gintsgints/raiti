# Information for lesson creators

Lesson files should be located at data directory. Lesson contents
are defined in index.yaml file. There is enumerated all lesson files.

Lesson files are located same data directory.

## Lesson page parameters

Each lesson file contains page entity list. Each page my contain next elements:

* title (mandatory) - Page title
* content (mandatory) - Page content, which my contain previous exercise results - '{{wpm}}' and '{{errors}}'
* show_keys (default to empty) - List of (col, row) for keys to show, when learning key positions.
* keyboard (default to no) - true/false weather onscreen keyboard should be shown.
* exercises (default to empty) - List of exercises (OneLineNoEnter or Multiline)
* content2 (default to zero string) - Help text to show at bottom.

### Content notes

* to specify two spaces at begining of sentences in multiline, you can use |2 specifier.
  (see Lesson 8 content for Tab key)
