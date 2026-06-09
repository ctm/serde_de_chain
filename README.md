# serde_de_chain

Provides the `SerdeDeChain` derive macro that lets a struct or enum be
deserialized normally or, on failure, from a previous representation.
Apply `#[derive(SerdeDeChain)]` together with `#[serde_de_chain(OldType)]`
and provide `impl From<OldType> for Self`. If `OldType` itself derives
`SerdeDeChain`, the chain extends indefinitely.

I wrote this because Mb2, a closed-source poker server and client,
uses `jsonb` columns to record various messages that are passed back
and forth between the client and server. Over time, the format of
these messages have changed, which doesn't affect game-play, but does
create problems for returning hand histories.

So far, I haven't found a better way to do what I want to do, so I
created this crate.

See `tests/integration.rs` for usage examples.
