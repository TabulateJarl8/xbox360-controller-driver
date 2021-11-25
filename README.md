# xbox360-controller-driver
Driver for the Xbox 360 controller written in Rust. I'm not entirely sure why you would ever want to use this, but I was so insanely bored that I thought this was a good idea.

## Platform
I've only tested this on Linux, but it might work on Windows/macOS. I do not plan to test this on either of the latter platforms.

## Building/Download
For most Linux users, you should be able to download the prebuilt binary from [here](https://nightly.link/TabulateJarl8/xbox360-controller-driver/workflows/rust/master/xboxdriver.zip) as long as you install xdotool with your package manager. You can also build the project yourself by cloning the repository and running `cargo build --release`.
