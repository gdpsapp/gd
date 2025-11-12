@0xb81f833b4c52b26f;

using import "color.capnp".Color;
using import "id.capnp".Id;
using import "level.capnp".LevelReference;
using import "option.capnp".Option;
using import "record.capnp".Record;
using import "statistics.capnp".Rating;
using import "time.capnp".Timestamp;
using import "user.capnp".UserReference;

struct CoreComment {
    id @0 :Id;
    author @1 :UserReference;
    content @2 :Text;
    color @3 :Color;
    rating @4 :Rating;
    createdAt @5 :Timestamp;
}

using UserComment = CoreComment;

struct LevelComment {
    core @0 :CoreComment;
    level @1: LevelReference;
    record @2 :Option(Record);
}
