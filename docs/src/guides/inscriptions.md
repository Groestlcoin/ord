Ordinal Inscription Guide
=========================

Individual gros can be inscribed with arbitrary content, creating
Groestlcoin-native digital artifacts that can be held in a Groestlcoin wallet and
transferred using Groestlcoin transactions. Inscriptions are as durable, immutable,
secure, and decentralized as Groestlcoin itself.

Working with inscriptions requires a Groestlcoin full node, to give you a view of
the current state of the Groestlcoin blockchain, and a wallet that can create
inscriptions and perform gro control when constructing transactions to send
inscriptions to another wallet.

Groestlcoin Core provides both a Groestlcoin full node and wallet. However, the Groestlcoin
Core wallet cannot create inscriptions and does not perform gro control.

This requires [`ord`](https://github.com/Groestlcoin/ord-groestlcoin), the ordinal utility. `ord`
doesn't implement its own wallet, so `ord wallet` subcommands interact with
Groestlcoin Core wallets.

This guide covers:

1. Installing Groestlcoin Core
2. Syncing the Groestlcoin blockchain
3. Creating a Groestlcoin Core wallet
4. Using `ord wallet receive` to receive gros
5. Creating inscriptions with `ord wallet inscribe`
6. Sending inscriptions with `ord wallet send`
7. Receiving inscriptions with `ord wallet receive`

Getting Help
------------

If you get stuck, try asking for help on the [Groestlcoin Discord
Server](https://discord.gg/vCKxQBz), or checking GitHub for relevant
[issues](https://github.com/Groestlcoin/ord-groestlcoin/issues).

Installing Groestlcoin Core
-----------------------

Groestlcoin Core is available from [groestlcoin.org](https://www.groestlcoin.org/groestlcoin-core-wallet/).

Making inscriptions requires Groestlcoin Core 24 or newer.

This guide does not cover installing Groestlcoin Core in detail. Once Groestlcoin Core
is installed, you should be able to run `groestlcoind -version` successfully from
the command line.

Configuring Groestlcoin Core
------------------------

`ord` requires Groestlcoin Core's transaction index.

To configure your Groestlcoin Core node to maintain a transaction
index, add the following to your `groestlcoin.conf`:

```
txindex=1
```

Or, run `groestlcoind` with `-txindex`:

```
groestlcoind -txindex
```

Syncing the Groestlcoin Blockchain
------------------------------

To sync the chain, run:

```
groestlcoind -txindex
```

…and leave it running until `getblockcount`:

```
groestlcoin-cli getblockcount
```

agrees with the block count on a block explorer like [the mempool.space block
explorer](https://mempool.space/). `ord` interacts with `groestlcoind`, so you
should leave `groestlcoind` running in the background when you're using `ord`.

Installing `ord`
----------------

The `ord` utility is written in Rust and can be built from
[source](https://github.com/Groestlcoin/ord-groestlcoin). Pre-built binaries are available on the
[releases page](https://github.com/Groestlcoin/ord-groestlcoin/releases).

You can install the latest pre-built binary from the command line with:

```sh
curl --proto '=https' --tlsv1.2 -fsLS https://raw.githubusercontent.com/Groestlcoin/ord-groestlcoin/master/install.sh | bash -s
```

Once `ord` is installed, you should be able to run:

```
ord --version
```

Which prints out `ord`'s version number.

Creating a Groestlcoin Core Wallet
------------------------------

`ord` uses Groestlcoin Core to manage private keys, sign transactions, and
broadcast transactions to the Groestlcoin network.

To create a Groestlcoin Core wallet named `ord` for use with `ord`, run:

```
ord wallet create
```

Receiving Gros
--------------

Inscriptions are made on individual gros, using normal Groestlcoin transactions
that pay fees in gros, so your wallet will need some gros.

Get a new address from your `ord` wallet by running:

```
ord wallet receive
```

And send it some funds.

You can see pending transactions with:

```
ord wallet transactions
```

Once the transaction confirms, you should be able to see the transactions
outputs with `ord wallet outputs`.

Creating Inscription Content
----------------------------

Gros can be inscribed with any kind of content, but the `ord` wallet only
supports content types that can be displayed by the `ord` block explorer.

Additionally, inscriptions are included in transactions, so the larger the
content, the higher the fee that the inscription transaction must pay.

Inscription content is included in transaction witnesses, which receive the
witness discount. To calculate the approximate fee that an inscribe transaction
will pay, divide the content size by four and multiply by the fee rate.

Inscription transactions must be less than 400,000 weight units, or they will
not be relayed by Groestlcoin Core. One byte of inscription content costs one
weight unit. Since an inscription transaction includes not just the inscription
content, limit inscription content to less than 400,000 weight units. 390,000
weight units should be safe.

Creating Inscriptions
---------------------

To create an inscription with the contents of `FILE`, run:

```
ord wallet inscribe --fee-rate FEE_RATE FILE
```

Ord will output two transactions IDs, one for the commit transaction, and one
for the reveal transaction, and the inscription ID. Inscription IDs are of the
form `TXIDiN`, where `TXID` is the transaction ID of the reveal transaction,
and `N` is the index of the inscription in the reveal transaction.

The commit transaction commits to a tapscript containing the content of the
inscription, and the reveal transaction spends from that tapscript, revealing
the content on chain and inscribing it on the first gro of the input that
contains the corresponding tapscript.

Wait for the reveal transaction to be mined. You can check the status of the
commit and reveal transactions using  [the esplora block
explorer](https://esplora.groestlcoin.org/).

Once the reveal transaction has been mined, the inscription ID should be
printed when you run:

```
ord wallet inscriptions
```

Parent Child Inscriptions
---------------------

A child inscription is an inscription that is a child of another inscription. See [provenance](../inscriptions/provenance.md) for more information.

get the parent inscription id `<PARENT_INSCRIPTION_ID>` from the output of `ord wallet inscriptions`

```
ord wallet inscribe --fee-rate FEE_RATE --parent <PARENT_INSCRIPTION_ID> CHILD_FILE"
```

And when you visit [the ordinals explorer](https://ordinals.groestlcoin.org/) at
`ordinals.groestlcoin.org/inscription/INSCRIPTION_ID`.

Sending Inscriptions
--------------------

Ask the recipient to generate a new address by running:

```
ord wallet receive
```

Send the inscription by running:

```
ord wallet send --fee-rate <FEE_RATE> <ADDRESS> <INSCRIPTION_ID>
```

See the pending transaction with:

```
ord wallet transactions
```

Once the send transaction confirms, the recipient can confirm receipt by
running:

```
ord wallet inscriptions
```

Receiving Inscriptions
----------------------

Generate a new receive address using:

```
ord wallet receive
```

The sender can transfer the inscription to your address using:

```
ord wallet send ADDRESS INSCRIPTION_ID
```

See the pending transaction with:
```
ord wallet transactions
```

Once the send transaction confirms, you can can confirm receipt by running:

```
ord wallet inscriptions
```
