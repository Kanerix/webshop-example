# Webshop example

This is a website created for fun and experience.


The website is created using the Rust framework Leptos. 
Even tho Leptos support SSR the website is currently using CSR.

# Running the website

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

- Tailwindcss: `npm i -g tailwindcss`

Install necessary toolchains and targets.

```
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknow`
````

## Development

### Using MPROCS

- Make sure you have `mprocs` installed (`cargo install mprocs`).

- Run the command `mprocs` in the root directory.

- Locate the website at http://localhost:8080

### Using only commands

- Make sure you have 

## Production

### Docker

- WIP