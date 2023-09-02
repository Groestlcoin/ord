Ordinal Theory FAQ
==================

What is ordinal theory?
-----------------------

Ordinal theory is a protocol for assigning serial numbers to gros, the
smallest subdivision of a groestlcoin, and tracking those gros as they are
spent by transactions.

These serial numbers are large numbers, like this 804766073970493. Every
gro, which is ¹⁄₁₀₀₀₀₀₀₀₀ of a groestlcoin, has an ordinal number.

Does ordinal theory require a side chain, a separate token, or changes to Groestlcoin?
----------------------------------------------------------------------------------

Nope! Ordinal theory works right now, without a side chain, and the only token
needed is groestlcoin itself.

What is ordinal theory good for?
--------------------------------

Collecting, trading, and scheming. Ordinal theory assigns identities to
individual gros, allowing them to be individually tracked and traded, as
curios and for numismatic value.

Ordinal theory also enables inscriptions, a protocol for attaching arbitrary
content to individual gros, turning them into groestlcoin-native digital
artifacts.

How does ordinal theory work?
-----------------------------

Ordinal numbers are assigned to gros in the order in which they are mined.
The first gro in the first block has ordinal number 0, the second has
ordinal number 1, and the last gro of the first block has ordinal number
4,999,999,999.

Gros live in outputs, but transactions destroy outputs and create new ones,
so ordinal theory uses an algorithm to determine how gros hop from the
inputs of a transaction to its outputs.

Fortunately, that algorithm is very simple.

Gros transfer in first-in-first-out order. Think of the inputs to a
transaction as being a list of gros, and the outputs as a list of slots,
waiting to receive a gro. To assign input gros to slots, go through
each gro in the inputs in order, and assign each to the first available
slot in the outputs.

Let's imagine a transaction with three inputs and two outputs. The inputs are
on the left of the arrow and the outputs are on the right, all labeled with
their values:

    [2] [1] [3] → [4] [2]

Now let's label the same transaction with the ordinal numbers of the gros
that each input contains, and question marks for each output slot. Ordinal
numbers are large, so let's use letters to represent them:

    [a b] [c] [d e f] → [? ? ? ?] [? ?]

To figure out which gro goes to which output, go through the input gros
in order and assign each to a question mark:

    [a b] [c] [d e f] → [a b c d] [e f]

What about fees, you might ask? Good question! Let's imagine the same
transaction, this time with a two gro fee. Transactions with fees send more
gros in the inputs than are received by the outputs, so to make our
transaction into one that pays fees, we'll remove the second output:

    [2] [1] [3] → [4]

The gros <var>e</var> and <var>f</var> now have nowhere to go in the
outputs:

    [a b] [c] [d e f] → [a b c d]

So they go to the miner who mined the block as fees. [The
BIP](https://github.com/Groestlcoin/ord-groestlcoin/blob/master/bip.mediawiki) has the details,
but in short, fees paid by transactions are treated as extra inputs to the
coinbase transaction, and are ordered how their corresponding transactions are
ordered in the block. The coinbase transaction of the block might look like
this:

    [SUBSIDY] [e f] → [SUBSIDY e f]

Where can I find the nitty-gritty details?
------------------------------------------

[The BIP!](https://github.com/Groestlcoin/ord-groestlcoin/blob/master/bip.mediawiki)

Why are gro inscriptions called "digital artifacts" instead of "NFTs"?
----------------------------------------------------------------------

An inscription is an NFT, but the term "digital artifact" is used instead,
because it's simple, suggestive, and familiar.

The phrase "digital artifact" is highly suggestive, even to someone who has
never heard the term before. In comparison, NFT is an acronym, and doesn't
provide any indication of what it means if you haven't heard the term before.

Additionally, "NFT" feels like financial terminology, and the both word
"fungible" and sense of the word "token" as used in "NFT" is uncommon outside
of financial contexts.

How do gro inscriptions compare to…
-----------------------------------

### Ethereum NFTs?

*Inscriptions are always immutable.*

There is simply no way to for the creator of an inscription, or the owner of an
inscription, to modify it after it has been created.

Ethereum NFTs *can* be immutable, but many are not, and can be changed or
deleted by the NFT contract owner.

In order to make sure that a particular Ethereum NFT is immutable, the contract
code must be audited, which requires detailed knowledge of the EVM and Solidity
semantics.

It is very hard for a non-technical user to determine whether or not a given
Ethereum NFT is mutable or immutable, and Ethereum NFT platforms make no effort
to distinguish whether an NFT is mutable or immutable, and whether the contract
source code is available and has been audited.

*Inscription content is always on-chain.*

There is no way for an inscription to refer to off-chain content. This makes
inscriptions more durable, because content cannot be lost, and scarcer, because
inscription creators must pay fees proportional to the size of the content.

Some Ethereum NFT content is on-chain, but much is off-chain, and is stored on
platforms like IPFS or Arweave, or on traditional, fully centralized web
servers. Content on IPFS is not guaranteed to continue to be available, and
some NFT content stored on IPFS has already been lost. Platforms like Arweave
rely on weak economic assumptions, and will likely fail catastrophically when
these economic assumptions are no longer met. Centralized web servers may
disappear at any time.

It is very hard for a non-technical user to determine where the content of a
given Ethereum NFT is stored.

*Inscriptions are much simpler.*

Ethereum NFTs depend on the Ethereum network and virtual machine, which are
highly complex, constantly changing, and which introduce changes via
backwards-incompatible hard forks.

Inscriptions, on the other hand, depend on the Groestlcoin blockchain, which is
relatively simple and conservative, and which introduces changes via
backwards-compatible soft forks.

*Inscriptions are more secure.*

Inscriptions inherit Groestlcoin's transaction model, which allow a user to see
exactly which inscriptions are being transferred by a transaction before they
sign it. Inscriptions can be offered for sale using partially signed
transactions, which don't require allowing a third party, such as an exchange
or marketplace, to transfer them on the user's behalf.

By comparison, Ethereum NFTs are plagued with end-user security
vulnerabilities. It is commonplace to blind-sign transactions, grant
third-party apps unlimited permissions over a user's NFTs, and interact with
complex and unpredictable smart contracts. This creates a minefield of hazards
for Ethereum NFT users which are simply not a concern for ordinal theorists.

*Inscriptions are scarcer.*

Inscriptions require groestlcoin to mint, transfer, and store. This seems like a
downside on the surface, but the raison d'etre of digital artifacts is to be
scarce and thus valuable.

Ethereum NFTs, on the other hand, can be minted in virtually unlimited
qualities with a single transaction, making them inherently less scarce, and
thus, potentially less valuable.

*Inscriptions do not pretend to support on-chain royalties.*

On-chain royalties are a good idea in theory but not in practice. Royalty
payment cannot be enforced on-chain without complex and invasive restrictions.
The Ethereum NFT ecosystem is currently grappling with confusion around
royalties, and is collectively coming to grips with the reality that on-chain
royalties, which were messaged to artists as an advantage of NFTs, are not
possible, while platforms race to the bottom and remove royalty support.

Inscriptions avoid this situation entirely by making no false promises of
supporting royalties on-chain, thus avoiding the confusion, chaos, and
negativity of the Ethereum NFT situation.

*Inscriptions unlock new markets.*

Groestlcoin's market capitalization and liquidity are greater than Ethereum's by a
large margin. Much of this liquidity is not available to Ethereum NFTs, since
many Groestlcoiners prefer not to interact with the Ethereum ecosystem due to
concerns related to simplicity, security, and decentralization.

Such Groestlcoiners may be more interested in inscriptions than Ethereum NFTs,
unlocking new classes of collector.

*Inscriptions have a richer data model.*

Inscriptions consist of a content type, also known as a MIME type, and content,
which is an arbitrary byte string. This is the same data model used by the web,
and allows inscription content to evolve with the web, and come to support any
kind of content supported by web browsers, without requiring changes to the
underlying protocol.

Inscriptions for…
-----------------

### Artists

*Inscriptions are on Groestlcoin.* Groestlcoin is the digital currency with the highest
status and greatest chance of long-term survival. If you want to guarantee that
your art survives into the future, there is no better way to publish it than as
inscriptions.

*Cheaper on-chain storage.* At $0,40 per GRS and the minimum relay fee of 1
gro per vbyte, publishing inscription content costs $0.0025 per 1 million
bytes.

*Inscriptions are early!* Inscriptions are still in development, and have not
yet launched on mainnet. This gives you an opportunity to be an early adopter,
and explore the medium as it evolves.

*Inscriptions are simple.* Inscriptions do not require writing or understanding
smart contracts.

*Inscriptions unlock new liquidity.* Inscriptions are more accessible and
appealing to groestlcoin holders, unlocking an entirely new class of collector.

*Inscriptions are designed for digital artifacts.* Inscriptions are designed
from the ground up to support NFTs, and feature a better data model, and
features like globally unique symbols and enhanced provenance.

*Inscriptions do not support on-chain royalties.* This is negative, but only
depending on how you look at it. On-chain royalties have been a boon for
creators, but have also created a huge amount of confusion in the Ethereum NFT
ecosystem. The ecosystem now grapples with this issue, and is engaged in a
race to the bottom, towards a royalties-optional future. Inscriptions have no
support for on-chain royalties, because they are technically infeasible. If you
choose to create inscriptions, there are many ways you can work around this
limitation: withhold a portion of your inscriptions for future sale, to benefit
from future appreciation, or perhaps offer perks for users who respect optional
royalties.

### Collectors

*Inscriptions are simple, clear, and have no surprises.* They are always
immutable and on-chain, with no special due diligence required.

*Inscriptions are on Groestlcoin.* You can verify the location and properties of
inscriptions easily with Groestlcoin full node that you control.

### Groestlcoiners

Let me begin this section by saying: the most important thing that the Groestlcoin
network does is decentralize money. All other use-cases are secondary,
including ordinal theory. The developers of ordinal theory understand and
acknowledge this, and believe that ordinal theory helps, at least in a small
way, Groestlcoin's primary mission.

Digital artifacts have merit.
There are, of course, a great deal of NFTs that are ugly, stupid, and
fraudulent. However, there are many that are fantastically creative, and
creating and collecting art has been a part of the human story since its
inception, and predates even trade and money, which are also ancient
technologies.

Groestlcoin provides an amazing platform for creating and collecting digital
artifacts in a secure, decentralized way, that protects users and artists in
the same way that it provides an amazing platform for sending and receiving
value, and for all the same reasons.

Ordinals and inscriptions increase demand for Groestlcoin block space, which
increase Groestlcoin's security budget, which is vital for safeguarding Groestlcoin's
transition to a fee-dependent security model, as the block subsidy is halved
into insignificance.

Inscription content is stored on-chain, and thus the demand for block space for
use in inscriptions is unlimited. This creates a buyer of last resort for *all*
Groestlcoin block space. This will help support a robust fee market, which ensures
that Groestlcoin remains secure.

Inscriptions also counter the narrative that Groestlcoin cannot be extended or used
for new use-cases. Inscriptions provide a counter argument which is easy to
understand, and which targets a popular and proven use case, NFTs, which
makes it highly legible.

If inscriptions prove, as the authors hope, to be highly sought after digital
artifacts with a rich history, they will serve as a powerful hook for Groestlcoin
adoption: come for the fun, rich art, stay for the decentralized digital money.

Inscriptions are an extremely benign source of demand for block space. Unlike,
for example, stablecoins, which potentially give large stablecoin issuers
influence over the future of Groestlcoin development, or DeFi, which might
centralize mining by introducing opportunities for MEV, digital art and
collectables on Groestlcoin, are unlikely to produce individual entities with
enough power to corrupt Groestlcoin. Art is decentralized.

Inscription users and service providers are incentivized to run Groestlcoin full
nodes, to publish and track inscriptions, and thus throw their economic weight
behind the honest chain.

Ordinal theory and inscriptions do not meaningfully affect Groestlcoin's
fungibility. Groestlcoin users can ignore both and be unaffected.

We hope that ordinal theory strengthens and enriches groestlcoin, and gives it
another dimension of appeal and functionality, enabling it more effectively
serve its primary use case as humanity's decentralized store of value.
