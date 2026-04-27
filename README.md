# logreview-rs

logreview-rs is a work-in-progress (WIP) Rust program that will be one day
useful for reviewing logs from popular web servers.

For a working version of this program, try the
[LogReview](https://github.com/TechnologyClassroom/LogReview) project or the
other few that are recommended in that README.

I am using this project to learn Rust. You can ignore this project for now or
try to help me.

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

Create a `log` directory or change the configuration file to point to your web
server log.

    mkdir -p log

Place a web server log in the `log` directory. This is a potential example of
how to copy a log file from NGINX, but you may have to adjust or use `scp` to
pull from a server.

    cp /var/log/nginx/access.log log/

If you do not want to supply your own log files, you can import some mock logs
that I made as a submodule.

    git submodule update --init --recursive

Run the program.

    ./target/debug/logreview-rs
    # or
    cargo run
