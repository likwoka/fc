# fc
- Toy program to learn about Rust (2020 Oct)
- takes an numeric input, converts to C or F
- a command line program and a web application
- using rust, actix_web
- separated into 3 modules -- the core logic, the CLI, the web app.

## Thoughts on rust
- prefer coding style leads you to a more robust application by using enum, option -- they force you to think and cover every branch of logic
- strong typing does help to reduce a lot of conversion bugs (ex: from user input string to your type) 

## Thoughts on rust/actix for website
- if you need performance -- sure
- compile web app into 1 binary (even include askama templates) -- nice for deployment
- I have my html template and css in askama templates -- that means tweaking css style needs a compile and deploy ... don't use this style for real web app (ie, host assets such as css in a static folder)
- Dev cycle -- python is definitely faster than rust/actix ... for small toy app I am ok
