# Install EOS

To test and deploy smart contracts you will want to have a local EOS node running. The easiest way to setup a node is with Docker. See the [official Docker quickstart guide](https://developers.eos.io/eosio-nodeos/docs/docker-quickstart) for instructions.

We recommend using `docker-compose` to manage `nodeos` and `keosd` containers. You can download the official [`docker-compose-latest.yml`](https://raw.githubusercontent.com/EOSIO/eos/master/Docker/docker-compose-latest.yml) file and start the containers using these commands:

```sh
wget https://raw.githubusercontent.com/EOSIO/eos/master/Docker/docker-compose-latest.yml
docker volume create --name=nodeos-data-volume
docker volume create --name=keosd-data-volume
docker-compose -f docker-compose-latest.yml up
```

**Note #1!** If you are using `cleos` within a Docker container, you need to mount your project directory as a volume so that `cleos` can deploy your files. If you're using Docker Compose, add your project directory to the `volumes` section of the `keosd` container like so (abbreviated):

```yaml
services:
    keosd:
        volumes:
            - ./:mnt/dev/project:ro
```

**Note #2!** If you are expecting to see console output from `nodeos` then be sure to add `--contracts-console` to the end of the `nodeosd` command like so (abbreviated):

```yaml
services:
    nodeosd:
        command: /opt/eosio/bin/nodeosd.sh ... --contracts-console
```
