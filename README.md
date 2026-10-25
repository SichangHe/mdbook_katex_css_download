# `mdbook-katex` CSS Download
(authored by human unless marked 🤖)

Executable to download static CSS and fonts for `mdbook-katex`.

🤖 Downloads KaTeX 0.16.4 to match the renderer bundled with mdbook-katex.
The CSS and renderer versions must match; newer CSS can break formula layout.

[Self-host KaTeX CSS and fonts](https://github.com/lzanini/mdbook-katex#self-host-katex-css-and-fonts).

## Usage

1. Install this executable

    ```shell
    cargo install mdbook_katex_css_download
    ```

1. `cd` to the root of your mdBook project.
1. Run `mdbook_katex_css_download`.
1. If you see no error, you are good to go.
    If it panics and the first error message suggests connection failure,
    it might just be a network problem, try again.
1. If it panics and the error is not connection failure,
    or it fails again and again,
    [open an issue](https://github.com/SichangHe/mdbook_katex_css_download/issues/new).

## Acknowledgements

A large portion of the code here is extracted from [`mdbook-katex`](https://github.com/lzanini/mdbook-katex).
The original author of the extracted code is Matthewacon.
[Their last commit](https://github.com/lzanini/mdbook-katex/commit/9452fdecfa37dd1d2569fcc5fd88dd9498903474).
