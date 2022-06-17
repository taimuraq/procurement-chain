# Hyperledger Grid
This project is using Hyperledger Grid source code: https://github.com/hyperledger/grid

To run default example:
$ docker-compose -f examples/splinter/docker-compose.yaml pull generate-registry db-alpha scabbard-cli-alpha splinterd-alpha

docker-compose -f examples/splinter/docker-compose.yaml build --pull

$ docker-compose -f examples/splinter/docker-compose.yaml up

To run custom Vroozi network with 6 nodes:

$ docker-compose -f examples/splinter/hackathon-docker-compose.yaml

Once the nodes are up you can set up Circuits, Pikes and Grid items such as Schemas, Products and POs. 

# To set up a circuit (sample commands)


docker exec gridd-alpha cat /etc/grid/keys/gridd.pub

<copy key>

docker exec -it splinterd-alpha bash

echo "{key}" > gridd.pub

splinter circuit propose \
   --key /registry/alpha.priv \
   --url http://splinterd-alpha:8085  \
   --node alpha-node-000::tcps://splinterd-alpha:8044 \
   --node beta-node-000::tcps://splinterd-beta:8044 \
   --service gsAA::alpha-node-000 \
   --service gsBB::beta-node-000 \
   --service-type *::scabbard \
   --management grid \
   --service-arg *::admin_keys=$(cat gridd.pub) \
   --service-peer-group gsAA,gsBB


## More Information

- [Hyperledger Grid website](https://grid.hyperledger.org)
- [Documentation](https://grid.hyperledger.org/docs/)



## License

Hyperledger Grid software is licensed under the [Apache License Version
2.0](LICENSE) software license.

The Hyperledger Grid documentation in the
[grid-docs](https://github.com/hyperledger/grid-docs) repository is licensed
under a Creative Commons Attribution 4.0 International License (CC BY 4.0).
You may obtain a copy of the license at
<http://creativecommons.org/licenses/by/4.0/>.
