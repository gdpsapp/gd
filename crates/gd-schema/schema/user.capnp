@0x83252853124c73c2;

using import "id.capnp".ColorId;
using import "id.capnp".IconId;
using import "id.capnp".Id;
using import "id.capnp".LongIconId;
using import "id.capnp".RoleId;
using import "info.capnp".DemonInfo;
using import "info.capnp".LevelInfo;
using import "info.capnp".PlatformerInfo;
using import "option.capnp".Option;
using import "record.capnp".Record;
using import "statistics.capnp".CreatorPoints;
using import "statistics.capnp".Demons;
using import "statistics.capnp".Diamonds;
using import "statistics.capnp".Moons;
using import "statistics.capnp".OptionPlace;
using import "statistics.capnp".Rank;
using import "statistics.capnp".RewardCoins;
using import "statistics.capnp".SecretCoins;
using import "statistics.capnp".Stars;
using import "statistics.capnp".UserCoins;
using import "time.capnp".Timestamp;
using import "values.capnp".EnumValue;

struct UserReference {
    id @0 :Id;
    name @1 :Text;
    accountId @2 :Id;
}

struct UserStatistics {
    stars @0 :Stars;
    moons @1 :Moons;
    demons @2 :Demons;
    diamonds @3 :Diamonds;
    userCoins @4 :UserCoins;
    secretCoins @5 :SecretCoins;
    creatorPoints @6 :CreatorPoints;
    rank @7 :Rank;
    demonInfo @8 :DemonInfo;
    levelInfo @9 :LevelInfo;
    platformerInfo @10 :PlatformerInfo;
}

struct UserCosmetics {
    color1Id @0 :ColorId;
    color2Id @1 :ColorId;
    color3Id @2 :ColorId;
    glow @3 :Bool;
    iconType @4 :EnumValue;
    iconId @5 :LongIconId;
    cubeId @6 :LongIconId;
    shipId @7 :IconId;
    ballId @8 :IconId;
    ufoId @9 :IconId;
    waveId @10 :IconId;
    robotId @11 :IconId;
    spiderId @12 :IconId;
    swingId @13 :IconId;
    jetpackId @14 :IconId;
    explosionId @15 :IconId;
    streakId @16 :IconId;
}

struct UserStates {
    messageState @0 :EnumValue;
    friendRequestState @1 :EnumValue;
    commentState @2 :EnumValue;
    friendState @3 :EnumValue;
}

struct UserSocials {
    youtube @0 :Option(Text);
    x @1 :Option(Text);
    twitch @2 :Option(Text);
    discord @3 :Option(Text);
}

struct UserLeaderboard {
    record @0 :Record;
    coins @1 :RewardCoins;
    recordedAt @2 :Timestamp;
}

struct User {
    reference @0 :UserReference;
    roleId @1 :RoleId;
    banned @2 :Bool;
    statistics @3 :Option(UserStatistics);
    cosmetics @4 :Option(UserCosmetics);
    states @5 :Option(UserStates);
    socials @6 :Option(UserSocials);
    place @7 :OptionPlace;
    leaderboard @8 :Option(UserLeaderboard);
}
