@0xbbf49df9a6501603;

using import "statistics.capnp".RewardStars;
using import "statistics.capnp".RewardMoons;

struct Reward {
    union {
        stars @0 :RewardStars;
        moons @1 :RewardMoons;
    }
}
