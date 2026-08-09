# haxball-live-protocol

Versioned protobuf-compatible messages for reconstructing the visible state of
a live HaxBall room. The crate is provider-neutral and program-neutral.

The source emits a full checkpoint, including a canonical zero-frame HBR2
bootstrap for arbitrary custom stadiums, when a consumer attaches or static
room configuration changes. Between checkpoints it emits bounded dynamic frames
containing game, player, and disc state. Every frame carries an epoch,
monotonic sequence, source tick, and source timestamp.

The contract intentionally excludes room passwords, player auth and connection
strings, IP addresses, output-provider credentials, and program-private state.
