# Mainmatter Website Mailer

This is a simple email sending service used to deliver mails sent via the
[contact form on mainmatter.com](http://mainmatter.com/contact/). It's written
in Rust using [`workers-rs`](https://github.com/cloudflare/workers-rs) and runs
as a worker on
[Cloudflare's edge infrastructure](https://www.cloudflare.com/network/).

## Usage

The project is based on the
[template for worker-rs](https://github.com/cloudflare/rustwasm-worker-template).
The main code is in the `src/lib.rs` file.

The [`wrangler` package](https://github.com/cloudflare/wrangler2) is used to run
the worker locally:

```bash
$ pnpm run dev
```

as well as to deploy a new version:

```bash
$ pnpm run deploy
```

## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is
meant to be executed as compiled WebAssembly, and as such so **must** all the
code you write and depend upon. All crates and modules used in Rust-based
Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the
[`workers-rs`](https://github.com/cloudflare/workers-rs) project README.

## Copyright

Copyright &copy; 2019-2022 Mainmatter GmbH (https://mainmatter.com), released
under the
[Creative Commons Attribution-NonCommercial 4.0 International license](https://creativecommons.org/licenses/by-nc/4.0/).
