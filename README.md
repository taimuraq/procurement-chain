# Hyperledger Grid
This project is using Hyperledger Grid source code: https://github.com/hyperledger/grid

To run default example:
$ docker-compose -f examples/splinter/docker-compose.yaml pull generate-registry db-alpha scabbard-cli-alpha splinterd-alpha

docker-compose -f examples/splinter/docker-compose.yaml build --pull

$ docker-compose -f examples/splinter/docker-compose.yaml up

To run custom Vroozi network with 6 nodes:

$ docker-compose -f examples/splinter/hackathon-docker-compose.yaml

Once the nodes are up you can set up Circuits, Pikes and Grid items such as Schemas, Products and POs. 


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
