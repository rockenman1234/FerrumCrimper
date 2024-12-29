<div align="center">

<img src="https://github.com/rockenman1234/FerrumCrimper/blob/main/docs/assets/ferris.png?raw=true" width="200px" align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

[![Rust](https://github.com/rockenman1234/FerrumCrimper/actions/workflows/rust.yml/badge.svg)](https://github.com/rockenman1234/FerrumCrimper/actions/workflows/rust.yml)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/fecr)](https://crates.io/crates/fecr)
[![GitHub License](https://img.shields.io/github/license/rockenman1234/ferrumcrimper)](https://github.com/rockenman1234/FerrumCrimper/blob/main/LICENSE.txt)


# FerrumCrimper
A fast, Rust-powered CLI tool for efficient file management, compression, and archival with support for multiple formats and secure data handling.

</div>

> [!CAUTION]
> This project is still in active development, use at your own risk! Multithreading support still a work in progress.

## Installation 🦾

To install FerrumCrimper, you can either use binaries or build from source.

### Using Cargo (Recommended)
If you have cargo installed, simply run the command below to install FerrumCrimper on your system!
```
cargo install fecr
```
[Be sure to check out or crates.io page here!](https://crates.io/crates/fecr)

#### Looking For Pre-Built Binaries?
Unfortunately, we do not have pre-built binaries available at this time. We recommend building from source using the instructions below if you are unable to use Crates.io.

<!--- 
- Visit the [Releases page](https://github.com/rockenman1234/ferrumcrimper/releases) to download the latest version for your platform.
- Extract the downloaded file for your operating system, and place it in a directory included in your system’s `PATH`.
--->

## Build From Source 🤓
1. Ensure that you have Rust and Cargo installed. [You can install Rust here](https://www.rust-lang.org/).
2. Clone the repository:
  ```
   git clone https://github.com/rockenman1234/ferrumcrimper.git
  ```
3. Navigate to this folder using the `cd` command.
4. If you have `make` installed, you may use the following command to compile binaries for your operating system. You may then
place them in a directory included in your system’s `PATH`. Compiled binaries will be located under `./target/release/`
  ```
  make
  ```
5. *(Optional)* If you do not have `make` installed, you can run the following commands yourself using cargo as a substitute:
  ```
  cargo build --release --verbose
  ./target/release/fecr --help
  ```

## Why Use FerrumCrimper? ℹ️

FerrumCrimper is designed to provide efficient file management and compression with these benefits:

- __Speed and Performance:__ Built with Rust, FerrumCrimper delivers fast compression and extraction, optimized for low latency and efficient resource usage. It also supports threaded operation, enabling faster speeds by leveraging multiple cores for parallel processing.
- __Simple Command-Line Interface:__ FerrumCrimper’s straightforward CLI makes it easy to zip or unzip files with just a few commands.
- __Secure Data Handling:__ With strong Rust-based implementations, the tool ensures safe and secure handling of data throughout compression and archival processes.
- __Multi-Format Support:__ FerrumCrimper supports various file formats, making it a versatile tool for a variety of workflows.

## Supported File Formats ⚙️
> [!NOTE]
> This project is still in active development, as of `v0.0.2` the archiving and unarchiving of zip files is suppprted.
> As of `0.0.5`, the archiving and unarchiving of tar files is supported.
> See the usage below for more information.
> Run `fecr -h` for more info.

FerrumCrimper plans to support a wide range of file formats for both compression and extraction:

- ZIP: Zip Archive and Unarchive Support:
  - Use `--zip` or `-z` to compress a directory.
  - Use `--unzip` or `-uz` to unextract a file.
  - Use `--name` or `-n` to specify the name of the output.
  - Use `--output` or `-o` to specify the output directory.
  - Use `--level` or `-l` to specify the compression level.
    - The following compression levels are supported:
      - Deflated levels range from 0 to 9. Default is 6.
      - Bzip2 levels range from 0 to 9. Default is 6.
      - Zstd levels range from -7 to 22, with zero 
        being mapped to the level. Default is 3.
  - Use `--compression` or `-c` to specify the compression method.
    - The following compression methods are supported:
      - bzip2 (bzip)
      - deflate (default)
      - zstd (z)
- TAR: Tar archive and unarchive support:
  - Use `--tar` or `-t` to compress a directory.
  - Use `--untar` or `-ut` to unextract a file.
  - Use `--name` or `-n` to specify the name of the output.
  - Use `--output` or `-o` to specify the output directory.
- GZ: Gzip support coming soon!
- 7Z: 7zip support coming soon!
- BZIP2: BZip2 support coming soon!
- XZ: Xz support comming soon!
- Others: Additional formats will be supported in future updates.

## License 👨‍⚖️

[This project is licensed under the GNU GPLv2 License](https://github.com/rockenman1234/FerrumCrimper/blob/main/LICENSE.txt). 

## Contributing 🫶
We love and welcome contributions! To get started:
1. Fork this repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes, and test thoroughly.
4. Submit a pull request with a description of your changes, and we'll take a look - thanks!

[Please read our code of conduct before submitting a pull request.](https://github.com/rockenman1234/FerrumCrimper/blob/main/CODE_OF_CONDUCT.md)
