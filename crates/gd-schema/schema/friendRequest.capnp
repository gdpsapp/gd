@0xafcfecace976de47;

using import "id.capnp".Id;
using import "time.capnp".Timestamp;
using import "user.capnp".UserReference;
using import "values.capnp".EnumValue;

struct FriendRequest {
    id @0 :Id;
    user @1 :UserReference;
    type @2 :EnumValue;
    content @3 :Text;
    createdAt @4 :Timestamp;
    read @5 :Bool;
}
