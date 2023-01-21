# Taiga Tools

Collection of tools to improve my workflow with taiga. 

## Structure
The taiga_lib will be the central library. It will be used as an "abstraction" for the API, exposing functions that can be used in the other "applications". 

## Use cases
The first application will be a CLI. My personal use of this CLI will be to integrate this with MS Outlook. Using VBA in Outlook, I will create triggers, so that when a action like a mail is flagged or categorised, the macro will call the CLI with some arguments, the CLI will then handle the actions. Using Macros in Outlook is probably not the best solution with Add-Ins and Ms "Power Automate" existing, but it will work. Using the library, it will be very easy to create your own applications or plugins. 

## Rust
The reason for the project existance is that I want to learn the rust language. Therefore as much code as possible will be in written in rust. 