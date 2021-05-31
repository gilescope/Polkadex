FROM ubuntu:latest
WORKDIR .
RUN apt-get update && apt-get upgrade -y
COPY ./polkadex-thea-node ./polkadex-thea-node
COPY ./customSpecRaw.json ./customSpecRaw.json
CMD exec ./polkadex-thea-node --chain ./customSpecRaw.json \
                         --alice \
                         --port 30333 \
                         --ws-port 9945 \
                         --rpc-port 9933 \
#                         --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
                         --validator \
                         --rpc-methods Unsafe \
                         --name thea-alice
EXPOSE 30333
EXPOSE 9945
EXPOSE 9933