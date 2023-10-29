# version of the rust
FROM rust:1.70 

# a new folder to map to the file system
WORKDIR /zkp-server

# copy all the code from the file system to the docker image
COPY . .

RUN apt update
# the -y is to answer YES if it ask for something
RUN apt install -y protobuf-compiler

# execute a cargo run
RUN cargo build --release --bin server --bin client


# docker compose build zkpserver  
