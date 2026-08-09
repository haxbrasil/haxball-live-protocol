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

## Compatibility

- Every envelope declares one protocol version. Consumers reject versions they
  do not implement before applying any state.
- New optional protobuf fields may be added within a version when older
  consumers can safely ignore them. Changing the meaning, type, or tag of an
  existing field requires a new protocol version.
- A new epoch always begins with a checkpoint. A checkpoint may also replace a
  damaged sequence window within the same epoch. Dynamic frames are valid only
  when their sequence is contiguous and they reference the installed
  checkpoint.
- Producers and consumers must enforce the published frame, stadium, player,
  and disc limits before allocating or applying state.
- Release consumers first, then producers. Producers may emit a new protocol
  version only after every intended consumer advertises support for it.

The test suite exercises generated valid frames and arbitrary malformed byte
streams in addition to fixed compatibility examples.
