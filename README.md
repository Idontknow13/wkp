# :newspaper: wkp

A CLI tool designed to fetch Wikipedia excerpts written in Rust.

![Demo GIF](/assets/demo.gif)

## Usage:

- Open Help

`$ wkp -h`

- Fetch an excerpt from the Simple Wiki page intro for *Canada*

`$ wkp -t "Canada"`

- Fetch the whole Simple Wiki page intro for *Philosophy*

`$ wkp -t "Philosophy" --whole`

- Fetch an excerpt from the English Wiki page intro for *Francis Fitzgerald*

`$ wkp -t "Francis Fitzgerald" -s en`

- Fetch the excerpts of pages: *Dog* and *Cat*

`$ wkp -t Dog -t Cat`
