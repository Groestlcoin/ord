Ordinal Explorer
================

The `ord` binary includes a block explorer. We host a instance of the block
explorer on mainnet at [ordinals.groestlcoin.org](https://ordinals.groestlcoin.org), and on signet at
[ordinals-signet.groestlcoin.org](https://ordinals-signet.groestlcoin.org).

### Running The Explorer
The server can be run locally with:

`ord server`

To specify a port add the `--http-port` flag:

`ord server --http-port 8080`

To enable the JSON-API endpoints add the `--enable-json-api` or `-e` flag:

`ord --enable-json-api server`

To test how your inscriptions will look you can run:

`ord preview <FILE1> <FILE2> ...`

Search
------

The search box accepts a variety of object representations.

### Blocks

Blocks can be searched by hash, for example, the genesis block:

[00000ac5927c594d49cc0bdb81759d0da8297eb614683d3acb62f0703b639023](https://ordinals.groestlcoin.org/search/00000ac5927c594d49cc0bdb81759d0da8297eb614683d3acb62f0703b639023)

### Transactions

Transactions can be searched by hash, for example, the block 1 coinbase
transaction:

[cf72b5842b3528fd7f3065ba9e93c50a62e84f42b3b7b7a351d910b5e353b662](https://ordinals.groestlcoin.org/search/cf72b5842b3528fd7f3065ba9e93c50a62e84f42b3b7b7a351d910b5e353b662)

### Outputs

Transaction outputs can searched by outpoint, for example, the only output of
the genesis block coinbase transaction:

[3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:0](https://ordinals.groestlcoin.org/output/3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:0)

### Gros

Gros can be searched by integer, their position within the entire groestlcoin
supply:

[2099994106992659](https://ordinals.groestlcoin.org/search/2099994106992659)

By decimal, their block and offset within that block:

[481824.0](https://ordinals.groestlcoin.org/search/481824.0)

By degree, their cycle, blocks since the last halving, blocks since the last
difficulty adjustment, and offset within their block:

[1°0′0″0‴](https://ordinals.groestlcoin.org/search/1°0′0″0‴)

By name, their base 26 representation using the letters "a" through "z":

[ahistorical](https://ordinals.groestlcoin.org/search/ahistorical)

Or by percentile, the percentage of groestlcoin's supply that has been or will have
been issued when they are mined:

[100%](https://ordinals.groestlcoin.org/search/100%)
