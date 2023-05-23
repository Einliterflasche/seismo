# seismo

A lightweight script for automatically building and viewing LaTeX projects managed by [tectonic](https://tectonic-typesetting.github.io/en-US/).

### Usage
Simply create a new tectonic project (or `cd` into an existing one) and then start the script:
```console
$ tectonic -X new foo
$ cd foo
$ seismo
Started watching `/path/to/foo`
```

or alternatively:
```console
$ tectonic -X new foo
$ seismo foo 
Started watching `/path/to/foo`
```

### Installation
Simply use the `cargo install` command:
```console
$ cargo install seismo
```

### Prerequisites
 - a compatible `tectonic` version (tested versions: `0.12`)
 - `xdg-open` from [xdg-utils](https://freedesktop.org/wiki/Software/xdg-utils/) must be available

### Limitations
 - The script does _not_ listen for changes to `Tectonic.toml`. If you change the file, you will have to restart the script.
 - The script does _not_ close the file viewer after opening it. This _might_ lead to your pdf viewer reopening again and again.
