docker-compose down
# shellcheck disable=SC2046
docker rm -f $(docker ps -a -q)
# shellcheck disable=SC2046
docker volume rm $(docker volume ls -q)
docker rmi polkadex_validator-charlie:latest
docker rmi polkadex_validator-bob:latest
docker rmi polkadex_validator-alice:latest
