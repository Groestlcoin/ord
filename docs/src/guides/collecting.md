Collecting
==========

Currently, [ord](https://github.com/Groestlcoin/ord-groestlcoin/) is the only wallet supporting
gro-control and gro-selection, which are required to safely store and send rare
gros and inscriptions, hereafter ordinals.

The recommended way to send, receive, and store ordinals is with `ord`, but if
you are careful, it is possible to safely store, and in some cases send,
ordinals with other wallets.

As a general note, receiving ordinals in an unsupported wallet is not
dangerous. Ordinals can be sent to any groestlcoin address, and are safe as long as
the UTXO that contains them is not spent. However, if that wallet is then used
to send groestlcoin, it may select the UTXO containing the ordinal as an input, and
send the inscription or spend it to fees.

A [guide](./collecting/sparrow-wallet.md) to creating an `ord`-compatible wallet with [Sparrow-GRS Wallet](https://www.groestlcoin.org/sparrow-grs/), is available
in this handbook.

Please note that if you follow this guide, you should not use the wallet you
create to send GRS, unless you perform manual coin-selection to avoid sending
ordinals.
