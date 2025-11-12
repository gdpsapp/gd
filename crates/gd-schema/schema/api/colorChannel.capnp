@0xb440d4a7ebbc1e92;

using import "../color.capnp".Color;
using import "../values.capnp".EnumValue;
using import "hsv.capnp".Hsv;
using import "id.capnp".ColorChannelId;
using import "opacity.capnp".Opacity;
using import "opacity.capnp".OptionOpacity;

struct PlayerColorChannelOptions {
    playerColor @0 :EnumValue;
    opacity @1 :Opacity;
}

struct NormalColorChannelOptions {
    color @0 :Color;
    opacity @1 :Opacity;
}

struct CopiedColorChannelOptions {
    id @0 :ColorChannelId;
    hsv @1 :Hsv;
    opacity @2 :OptionOpacity;
}

struct ColorChannel {
    id @0 :ColorChannelId;
    blending @1 :Bool;
    union {
        player @2 :PlayerColorChannelOptions;
        normal @3 :NormalColorChannelOptions;
        copied @4 :CopiedColorChannelOptions;
    }
}
