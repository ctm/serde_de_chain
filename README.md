# serde_de_chain

Provides the `serde_de_chain!` macro that declares an enum in such a way
that it can be deserialized from historical representations that are
different from how it is currently declared.

I wrote this because Mb2, a closed-source poker server and client,
uses `jsonb` columns to record various messages that are passed back
and forth between the client and server. Over time, the format of
these messages have changed, which doesn't affect game-play, but does
create problems for returning hand histories.

So far, I haven't found a better way to do what I want to do, so I
created this crate.

See the documentation and the tests inside src/lib.rs for more info.
