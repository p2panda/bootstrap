FROM clux/muslrust:stable AS BUILDER

# Add source code
ADD . ./

# Build our application
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `bootstrap`
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=BUILDER \
    /volume/target/x86_64-unknown-linux-musl/release/bootstrap \
    /usr/local/bin/

ENV NETWORK_ID ""
ENV DATA_PATH /home
ENV RELAY_URL https://wasser.liebechaos.org

CMD /usr/local/bin/bootstrap --data-path $DATA_PATH --network-id $NETWORK_ID --relay-url $RELAY_URL
