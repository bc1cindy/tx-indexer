# Transaction Graph Analysis Framework

This project implements an analysis framework for the Bitcoin transaction graph that evaluates the privacy properties of both simulated and real-world privacy protocols.

The framework measures the quality of transaction-construction protocols (e.g., CoinJoin and PayJoin) using state-of-the-art wallet clustering techniques and subtransaction-level metrics. Our goal is to deliver a reusable suite of privacy heuristics that researchers and developers can apply to systematically evaluate how well a given protocol preserves privacy. But more generally, answers any question about the transaction graph.

Chain analysis firms likely already rely on similar tooling inside proprietary, closed-source systems. We aim to make these methods open source and accessible to the broader research and development community. Today, no widely available framework --BlockSci comes closest -- supports composable information flows that let analysts combine signals and build progressively more refined heuristics.

Finally, much of the recent CoinJoin literature measures privacy at the single-transaction level and ignores the surrounding transaction (sub)graph. However, intersection-attack research demonstrates that pre- and post-mix activity can significantly erode privacy as shown in work on intersection attacks (e.g., Goldfeder et al.).

The research direction is to treat on-chain privacy as an emergent property of transaction-graph structure, rather than as a per-transaction heuristic, and to analyze that structure using concepts from spectral graph analysis. Dense transaction construction (e.g. radix CoinJoins) provides strong local ambiguity, while [verifiable randomness in peer and coin selection](https://gist.github.com/nothingmuch/f5b9a559958c6116606d9da0d4d884f2
) induces global graph properties that drive rapid mixing and many plausible paths. Entropy captures the size of anonymity sets, but edge-differential-privacy–style parameters capture their robustness to information revelation; the open problem is interpreting these parameters meaningfully when the algorithm and data are fixed and ( \varepsilon ) must be estimated rather than chosen. Key open questions include how fragile large anonymity sets are under realistic edge deletions, how degree sequences evolve over time in the randomized subgraph, and how much information leaks through revealed preferences. [Simulations](https://github.com/payjoin/btsim) can cover most of the empirical work: generating transaction graphs under different randomized selection rules, estimating degree distributions and path counts, measuring stationary distributions, stress-testing edge removals, and validating whether the resulting graphs behave like expanders / superconcentrators with high probability.

## Contributing

See [`CONTRIBUTING.md`](.github/CONTRIBUTING.md).