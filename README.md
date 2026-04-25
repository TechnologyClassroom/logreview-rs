# logreview-rs
logreview-rs is a work-in-progress (WIP) Rust program that is useful for reviewing logs from popular web servers.

For a working version of this program, try the [LogReview](https://github.com/TechnologyClassroom/LogReview) project or the other few that are recommended in that README.

I am using this project to learn Rust. You can ignore this project for now or try to help me.

## How to try it

Update your repositories if you are on a Debian-based system.

    sudo apt update

Install git if you are on a Debian-based system.

    apt install -y git

Install Rust.

Clone this directory.

    git clone https://github.com/technologyclassroom/logreview-rs

Change to the logreview-rs directory.

    cd logreview-rs

Build dependencies and `logreview-rs` binary.

    cargo build

Run the program.

    ./target/debug/logreview-rs
    # or
    cargo run
