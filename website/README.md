# Website

The homepage is static HTML. The guide uses [mdBook](https://rust-lang.github.io/mdBook/).

Install mdBook if needed:

```sh
cargo install mdbook --version 0.5.4 --locked
```

Build the guide from the repository root:

```sh
mdbook build website/guide
```

Serve the homepage and guide together:

```sh
python3 -m http.server 8000 --directory website
```

Open <http://localhost:8000/>. The documentation buttons link to the generated guide in `guide/book/`.

Edit the guide in `guide/src/`, then rebuild it. Generated files in `guide/book/` are ignored by Git.

The [Pages workflow](../.github/workflows/pages.yml) deploys the homepage and guide when website files change on `master`. It also supports manual runs from the Actions tab.

In the repository's **Settings → Pages**, set **Source** to **GitHub Actions**.
