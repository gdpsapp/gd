@0xf23d96c5f3c74c0d;

using import "color.capnp".Color;
using import "id.capnp".Id;
using import "statistics.capnp".RewardCoins;
using import "statistics.capnp".RewardStars;
using import "values.capnp".EnumValue;

struct MapPack {
    id @0 :Id;
    name @1 :Text;
    levelIds @2 :List(Id);
    stars @3 :RewardStars;
    coins @4 :RewardCoins;
    difficulty @5 :EnumValue;
    color @6 :Color;
}
