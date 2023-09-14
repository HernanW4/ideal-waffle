# 1st Stage
# use the main official rust docker image as our builder
FROM rust as builder

#copy the app into the docker image
COPY . /app

# set the work directory
WORKDIR /app

# build the app
RUN cargo build --release

# use google distroless as runtime image
FROM ubuntu

# copy app from buidler
COPY --from=builder /app/target/release/ideal-waffle /app/ideal-waffle
WORKDIR /app


# start application
CMD ["./ideal-waffle"]
