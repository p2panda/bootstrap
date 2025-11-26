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
ENV PRIVATE_KEY "/home/private-key.txt"
ENV RELAY_URL "https://euc1-1.relay.n0.iroh-canary.iroh.link."

CMD /usr/local/bin/bootstrap --private-key $PRIVATE_KEY --network-id $NETWORK_ID --relay-url $RELAY_URL
