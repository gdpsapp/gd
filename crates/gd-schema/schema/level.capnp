@0xea1c9d34d3f009fd;

using import "capacity.capnp".Capacity;
using import "id.capnp".Id;
using import "id.capnp".TimelyId;
using import "option.capnp".Option;
using import "password.capnp".Password;
using import "reward.capnp".Reward;
using import "song.capnp".SongReference;
using import "statistics.capnp".Downloads;
using import "statistics.capnp".ObjectCount;
using import "statistics.capnp".Rating;
using import "statistics.capnp".RewardCoins;
using import "statistics.capnp".RewardStars;
using import "statistics.capnp".Score;
using import "time.capnp".Duration;
using import "time.capnp".OptionTimestamp;
using import "time.capnp".TimeSteps;
using import "user.capnp".UserReference;
using import "values.capnp".EnumValue;
using import "version.capnp".Version;

struct LevelReference {
    id @0 :Id;
    name @1 :Text;
}

struct Level {
    reference @0 :LevelReference;
    description @1 :Text;
    creator @2 :UserReference;
    song @3 :SongReference;
    data @4 :Option(Data);
    version @5 :Version;
    downloads @6 :Downloads;
    gameVersion @7 :Version;
    rating @8 :Rating;
    length @9 :EnumValue;
    difficulty @10 :EnumValue;
    reward @11 :Reward;
    requestedReward @12 :Reward;
    score @13 :Score;
    rateType @14 :EnumValue;
    password @15 :Password;
    originalId @16 :Id;
    twoPlayer @17 :Bool;
    capacity @18 :Option(Capacity);
    coins @19 :RewardCoins;
    verifiedCoins @20 :Bool;
    lowDetail @21 :Bool;
    objectCount @22 :ObjectCount;
    createdAt @23 :OptionTimestamp;
    updatedAt @24 :OptionTimestamp;
    editorTime @25 :Duration;
    copiesTime @26 :Duration;
    timelyType @27 :EnumValue;
    timelyId @28 :TimelyId;
    timeSteps @29 :TimeSteps;
}
