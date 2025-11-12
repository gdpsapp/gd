@0xb01864dba3141a3b;

using import "../id.capnp".Id;
using import "../option.capnp".Option;
using import "../size.capnp".Size;
using import "artist.capnp".ArtistApi;
using import "priority.capnp".Priority;

struct SongApi {
    id @0 :Id;
    name @1 :Text;
    artist @2 :ArtistApi;
    size @3 :Size;
    url @4 :Option(Text);
    priority @5 :Priority;
}
