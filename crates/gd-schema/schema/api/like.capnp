@0xa64a59e494f00145;

using import "../id.capnp".Id;
using import "../id.capnp".OptionId;
using import "../values.capnp".EnumValue;

struct Like {
    type @0 :EnumValue;
    targetId @1 :Id;
    relatedId @2 :OptionId;
    liked @3 :Bool;
}
