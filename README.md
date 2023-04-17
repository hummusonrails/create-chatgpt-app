# Create ChatGPT App

Create ChatGPT apps with Rust and Rocket.

[![Crates.io](https://img.shields.io/crates/v/create_chatgpt_app_.svg)](https://crates.io/crates/create_chatgpt_app)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Contributing](https://img.shields.io/badge/Contributing-Guidelines-blue)](CONTRIBUTING.md)
[![Code of Conduct](https://img.shields.io/badge/Code%20of%20Conduct-Respectful-orange)](CODE_OF_CONDUCT.md)
[![Visitors](https://visitor-badge.glitch.me/badge?page_id=hummusonrails.create-chatgpt-app)](https://github.com/hummusonrails/create-chatgpt-app)
[![Made with Love](https://img.shields.io/badge/Made%20with-Love-ff69b4.svg)](https://shields.io/)

Create ChatGPT App works on macOS, Windows, and Linux.<br>
If something doesn’t work, please [file an issue](https://github.com/hummusonrails/create-chatgpt-app/issues/new).<br>
If you have questions or need help, please ask in [GitHub Discussions](https://github.com/hummusonrails/create-chatgpt-app/discussions).

## Quick Overview

```sh
cargo install create_chatgpt_app
create_chatgpt_app my-app
cd my-app
cargo run
```

Then open http://localhost:8000/ to see your app.

When you’re ready to deploy to production, create a release build with `cargo build --release`.

## Get Started Immediately

You don’t need to install or configure tools like Rocket or ChatGPT API.<br>
They are preconfigured and hidden so that you can focus on the code.

Create a project, and you’re good to go.

## Creating an App

You’ll need to have Rust and Cargo installed on your local development machine. We recommend using the latest stable version. You can use [rustup](https://rustup.rs/) to manage Rust versions.

To create a new app, follow these steps:

```sh
cargo install create_chatgpt_app
create_chatgpt_app my-app
```

It will create a directory called `my-app` inside the current folder.<br>
Inside that directory, it will generate the initial project structure:

```sh
my-app
├── README.md
├── Cargo.toml
├── src
│   ├── main.rs
│   ├── chatgpt.rs
└── templates
    └── index.html.hbs
```

No configuration or complicated folder structures, only the files you need to build your app.<br>
Once the installation is done, you can open your project folder:

```sh
cd my-app
```

Inside the newly created project, you can run some built-in commands:

### `cargo run`

Runs the app in development mode.<br>
Open http://localhost:8000 to view it in the browser.

The page will automatically reload if you make changes to the code.<br>
You will see the build errors and lint warnings in the console.

### `cargo build --release`

Builds the app for production to the target/release folder.<br>
It correctly optimizes the build for the best performance.

The build is minified and the filenames include the hashes.<br>

Your app is ready to be deployed.

## Contributing

Contributions are welcome! If you'd like to improve Create ChatGPT App or suggest new features, please submit a pull request or create an issue. More information on contributing to the project can be found in the [contributing guidelines](CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.