![Logo](https://raw.githubusercontent.com/ben-maclaurin/elde/main/logo.png)

Elde: a tiny, minimal static site generator for Emacs [Org Mode](https://orgmode.org/) users. It is heavily inspired by [Jekyll](https://jekyllrb.com/).

Why? To scratch a personal itch. Currently a WIP project. 

## Get started

First, clone this repo.

Then, in the project root:

```
cargo run
```

This will generate `/static` and `/org`. Elde will watch `/org` for changes and the output will be exported to `/static`.

I do plan to make a binary available here soon.

## Usage

You can apply custom CSS by amending the following line in `transform.rs`:

```
#+HTML_HEAD: <link rel=\"stylesheet\" type=\"text/css\" href=\"https://unpkg.com/sakura.css/css/sakura-earthly.css\" />\n
```

Configuration via a .toml is planned.

## Contributing

Contributions are welcome.

## License

MIT 
