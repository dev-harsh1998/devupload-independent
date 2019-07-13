# Devupload [![Codacy Badge](https://api.codacy.com/project/badge/Grade/73c7910f694b4502bf53e2b905868762)](https://www.codacy.com?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=dev-harsh1998/devupload-independent&amp;utm_campaign=Badge_Grade) [![Codeship Status for dev-harsh1998/devupload-independent](https://app.codeship.com/projects/b40c0f90-87b7-0137-e1fb-6207466b21af/status?branch=master)](https://app.codeship.com/projects/354235)

devupload is a standalone command line utility written in rust to pulverize the ftp uploading procedure to just few inputs i.e username and password, Yep and its done.

## How to use?
```
devupload --file-path path/to/file/to/be/uploaded.zip --androidfilehost
devupload --file-path path/to/file/to/be/uploaded.zip --basketbuild

Same could be achieved with
devupload -af path/to/file/to/be/uploaded.zip for androidfilehost
devupload -bf path/to/file/to/be/uploaded.zip for basketbuild
```

## Installation
You can simply install devupload by following these instructions, make sure you have `git` and `rust` installed.

```
# Clone the repository.
git clone https://github.com/dev-harsh1998/devupload-independent
# Navigate to cloned directory.
cd devupload
# Compile latest binary on your own machine!!.
cargo build --release
# Install it for direct access.
sudo mv target/release/devupload /usr/local/bin/
```

## Supported Platforms
- Windows (Rust installed)
- Linux x86
- Linux x86_64
- macOS

## TODO
- Add support for sourcefourge?

## Issue or feature request?
Please write about issues and feature request [here](https://github.com/dev-harsh1998/devupload-independent).