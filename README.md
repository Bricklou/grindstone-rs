<h1>
    <img src=".github/logo.svg" alt="grindstone-rs" align="center"/>
</h1>
<div align="center">
    <strong>
        Rust library to download and run Minecraft instances.
    </strong>
</div>

<br/>

<div align="center">
    <!-- Github Actions -->
    <img src="https://img.shields.io/github/workflow/status/bricklou/grindstone-rs/Grindstone-rs?style=flat-square" alt="actions status" /> 
    <!-- Dependency Status -->
    <a href="https://deps.rs/repo/github/bricklou/grindstone-rs">
        <img src="https://deps.rs/repo/github/bricklou/grindstone-rs/status.svg" alt="Dependency status" />
    </a>
    <!-- License -->
    <a href="https://opensource.org/licenses/MIT">
        <img src="https://img.shields.io/github/license/bricklou/grindstone-rs" />
    </a>
</div>

## Build the code

To build the library, the do the following command:
```sh
carbo build
```

## Run the example

You can run the example program from `examples/example.rs` using the command below. Don't forgot to add `RUST_LOG` environment variable to 
have the full logs:
```sh
RUST_LOG=grindstone-rs=trace,debug cargo run --example example
```

## Authors

- [@Bricklou](https://www.github.com/bricklou)
