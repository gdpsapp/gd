@0xc7c81742162b8987;

using import "percent.capnp".Percent;
using import "time.capnp".Duration;

struct Record {
    union {
        percent @0 :Percent;
        duration @1 :Duration;
    }
}
