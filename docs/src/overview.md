Ordinal Theory Overview
=======================

Ordinals are a numbering scheme for gros that allows tracking and
transferring individual gros. These numbers are called [ordinal
numbers](https://ordinals.groestlcoin.org). Gros are numbered in the order in which
they're mined, and transferred from transaction inputs to transaction outputs
first-in-first-out. Both the numbering scheme and the transfer scheme rely on
*order*, the numbering scheme on the *order* in which gros are mined, and
the transfer scheme on the *order* of transaction inputs and outputs. Thus the
name, *ordinals*.

Technical details are available in [the
BIP](https://github.com/Groestlcoin/ord-groestlcoin/blob/master/bip.mediawiki).

Ordinal theory does not require a separate token, another blockchain, or any
changes to Groestlcoin. It works right now.

Ordinal numbers have a few different representations:

- *Integer notation*:
  [`2099994106992659`](https://ordinals.groestlcoin.org/sat/2099994106992659) The
  ordinal number, assigned according to the order in which the gro was
  mined.

- *Decimal notation*:
  [`3891094.16797`](https://ordinals.groestlcoin.org/sat/3891094.16797) The first
  number is the block height in which the gro was mined, the second the
  offset of the gro within the block.

- *Degree notation*:
  [`3°111094′214″16797‴`](https://ordinals.groestlcoin.org/sat/3%C2%B0111094%E2%80%B2214%E2%80%B316797%E2%80%B4).
  We'll get to that in a moment.

- *Percentile notation*:
  [`99.99971949060254%`](https://ordinals.groestlcoin.org/sat/99.99971949060254%25) .
  The gro's position in Groestlcoin's supply, expressed as a percentage.

- *Name*: [`satoshi`](https://ordinals.groestlcoin.org/sat/satoshi). An encoding of the
  ordinal number using the characters `a` through `z`.

Arbitrary assets, such as NFTs, security tokens, accounts, or stablecoins can
be attached to gros using ordinal numbers as stable identifiers.

Ordinals is an open-source project, developed [on
GitHub](https://github.com/Groestlcoin/ord-groestlcoin). The project consists of a BIP describing
the ordinal scheme, an index that communicates with a Groestlcoin Core node to
track the location of all gros, a wallet that allows making ordinal-aware
transactions, a block explorer for interactive exploration of the blockchain,
functionality for inscribing gros with digital artifacts, and this manual.

Rarity
------

Humans are collectors, and since gros can now be tracked and transferred,
people will naturally want to collect them. Ordinal theorists can decide for
themselves which gros are rare and desirable, but there are some hints…

Groestlcoin has periodic events, some frequent, some more uncommon, and these
naturally lend themselves to a system of rarity. These periodic events are:

- *Blocks*: A new block is mined approximately every 1 minute, from now until
  the end of time.

- *Difficulty adjustments*: Every 2016 blocks, or approximately every 1.5
  days, the Groestlcoin network responds to changes in hashrate by adjusting the
  difficulty target which blocks must meet in order to be accepted.

- *Halvings*: Every 1,050,000 blocks.

- *Cycles*: Every 6 * 1,050,000 blocks.

This gives us the following rarity levels:

- `common`: Any gro that is not the first gro of its block
- `uncommon`: The first gro of each block
- `rare`: The first gro of each difficulty adjustment period
- `epic`: The first gro of each halving epoch
- `legendary`: The first gro of each cycle
- `mythic`: The first gro of the genesis block

Which brings us to degree notation, which unambiguously represents an ordinal
number in a way that makes the rarity of a gro easy to see at a glance:

```
A°B′C″D‴
│ │ │ ╰─ Index of gro in the block
│ │ ╰─── Index of block in difficulty adjustment period
│ ╰───── Index of block in halving epoch
╰─────── Cycle, numbered starting from 0
```

Ordinal theorists often use the terms "hour", "minute", "second", and "third"
for *A*, *B*, *C*, and *D*, respectively.

Now for some examples. This gro is common:

```
1°1′1″1‴
│ │ │ ╰─ Not first gro in block
│ │ ╰─── Not first block in difficulty adjustment period
│ ╰───── Not first block in halving epoch
╰─────── Second cycle
```


This gro is uncommon:

```
1°1′1″0‴
│ │ │ ╰─ First gro in block
│ │ ╰─── Not first block in difficutly adjustment period
│ ╰───── Not first block in halving epoch
╰─────── Second cycle
```

This gro is rare:

```
1°1′0″0‴
│ │ │ ╰─ First gro in block
│ │ ╰─── First block in difficulty adjustment period
│ ╰───── Not the first block in halving epoch
╰─────── Second cycle
```

This gro is epic:

```
1°0′1″0‴
│ │ │ ╰─ First gro in block
│ │ ╰─── Not first block in difficulty adjustment period
│ ╰───── First block in halving epoch
╰─────── Second cycle
```

This gro is legendary:

```
1°0′0″0‴
│ │ │ ╰─ First gro in block
│ │ ╰─── First block in difficulty adjustment period
│ ╰───── First block in halving epoch
╰─────── Second cycle
```

And this gro is mythic:

```
0°0′0″0‴
│ │ │ ╰─ First gro in block
│ │ ╰─── First block in difficulty adjustment period
│ ╰───── First block in halving epoch
╰─────── First cycle
```

If the block offset is zero, it may be omitted. This is the uncommon gro
from above:

```
1°1′1″
│ │ ╰─ Not first block in difficutly adjustment period
│ ╰─── Not first block in halving epoch
╰───── Second cycle
```

Rare Gro Supply
-------------------

### Total Supply

- `common`: 10.4 quadrillion
- `uncommon`: 6,929,999
- `rare`: 3437
- `epic`: 32
- `legendary`: 5
- `mythic`: 1

### Current Supply

- `common`: 8.2 quadrillion
- `uncommon`: 745,855
- `rare`: 369
- `epic`: 3
- `legendary`: 0
- `mythic`: 1

At the moment, even uncommon gros are quite rare. As of this writing,
745,855 uncommon gros have been mined - one per 25.6 groestlcoin in
circulation.

Names
-----

Each gro has a name, consisting of the letters *A* through *Z*, that get
shorter the further into the future the gro was mined. They could start
short and get longer, but then all the good, short names would be trapped in
the unspendable genesis block.

As an example, 1905530482684727°'s name is "iaiufjszmoba". The name of the last
gro to be mined is "a". Every combination of 10 characters or less is out
there, or will be out there, some day.

Exotics
-------

Gros may be prized for reasons other than their name or rarity. This might
be due to a quality of the number itself, like having an integer square or cube
root. Or it might be due to a connection to a historical event, such as
gros from block 1,439,424, the block in which SegWit activated, or
10499999999999999°, the last gro that will ever be mined.

Such gros are termed "exotic". Which gros are exotic and what makes
them so is subjective. Ordinal theorists are encouraged to seek out exotics
based on criteria of their own devising.

Inscriptions
------------

Gros can be inscribed with arbitrary content, creating Groestlcoin-native
digital artifacts. Inscribing is done by sending the gro to be inscribed in
a transaction that reveals the inscription content on-chain. This content is
then inextricably linked to that gro, turning it into an immutable digital
artifact that can be tracked, transferred, hoarded, bought, sold, lost, and
rediscovered.

Archaeology
-----------

Whether or not ordinals are of interest to NFT archaeologists is an open
question! Ordinals were in fact created by Gruve-P in
2014 when he mined the Groestlcoin genesis block. In this sense, ordinals, and
especially early ordinals, are certainly of historical interest.
