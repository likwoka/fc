# fc
- Toy program to learn about Rust (2020 Oct)
- takes an numeric input, converts to C or F
- a command line program (fc) and a server side web application (fcweb)
- using rust, actix-web
- This is structured as a workspace with 2 libraries, each with a binary: a command line app and a web app.
- Notice how the dependency is specified between fcweb (the web app) and fc (the cli app and also a library with logic) in fcweb's Cargo.toml

## To build (binary will be generate in target folder)
cargo build

## To run CLI with cargo (for convenience during development)
cargo run --bin fc 23
cargo run --bin fc 23C
cargo run --bin fc 23f

## To run web (for convenience during development)
cargo run --bin fcweb # default to 127.0.0.1:8080
cargo run --bin fcweb 127.0.0.1:8081

## Thoughts on rust
- Preferred coding style nudges you to a very robust application; using tools such as Enum, Option which you to think and cover every branch of logic
- Strong typing helps to reduce a lot of conversion bugs (ex: from user input string to your type)
- Low resistance in adding unit tests
- The above combined makes Rust application quite robust by default (You can still have logic bug, but the attack surface would be small without you thinking a lot about it)

## Thoughts on rust/actix-web for website
- if you need performance -- great
- compile web app into 1 binary (even include askama templates) -- nice for deployment
- I have my html template and css in askama templates -- that means tweaking css style needs a compile and deploy -- (Not what you want for rapid development..)
- Maybe host assets such as css in a static folder
