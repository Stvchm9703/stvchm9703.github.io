pull update from https://github.com/anyproto/any-block

pb-rs -I=protos/anytype -o=src/proto/anytype/snapshot.rs    protos/anytype/snapshot.proto

pb-rs -I=protos/anytype -o=src/proto/anytype/models.rs    protos/anytype/models.proto -s --owned
pb-rs -I=protos/anytype -o=src/proto/anytype/events.rs    protos/anytype/events.proto -s
pb-rs -I=protos/anytype -o=src/proto/anytype/changes.rs    protos/anytype/changes.proto -s
pb-rs -I=protos/anytype -o=src/proto/anytype/snapshot.rs    protos/anytype/snapshot.proto