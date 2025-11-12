@0xabc0efd0406f8f97;

using import "metadata.capnp".Metadata;
using import "object.capnp".Object;

struct Editor {
    header @0 :Metadata;
    objects @1 :List(Object);
}
