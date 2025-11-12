@0xaea4dea251a81afc;

using import "artist.capnp".Artist;
using import "id.capnp".Id;
using import "option.capnp".Option;
using import "size.capnp".Size;

struct SongReference {
    id @0 :Id;
    custom @1 :Bool;
}

struct Song {
    id @0 :Id;
    name @1 :Text;
    artist @2 :Artist;
    size @3 :Size;
    url @4 :Option(Text);
}
