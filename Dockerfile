FROM ubuntu:latest
WORKDIR .
RUN apt-get update && apt-get upgrade -y
COPY ./target/release/polkadex-thea-node ./polkadex-thea-node
COPY ./customSpecRaw.json ./customSpecRaw.json
ENV bootnodePeerId=""
ENV bootnodeIP=""
ENV validator=""
CMD exec ./polkadex-thea-node   --chain ./customSpecRaw.json \
                           --port 30333 \
                           --ws-port 9945 \
                           --rpc-port 9933 \
#                           --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
                           --validator \
                           --rpc-methods Unsafe \
                           --name $validator\
                           --bootnodes /ip4/$bootnodeIP/tcp/30333/p2p/$bootnodePeerId \
                           --$validator \
                           -lthea=trace
EXPOSE 30333
EXPOSE 9945
EXPOSE 9933