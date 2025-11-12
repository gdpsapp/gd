@0x94465e9a2de58adc;

using import "../time.capnp".Timestamp;
using import "../values.capnp".EnumValue;
using import "colorChannel.capnp".ColorChannel;
using import "guidelines.capnp".Guidelines;
using import "id.capnp".VisualId;

struct Metadata {
    gameMode @0 :EnumValue;
    miniMode @1 :Bool;
    speed @2 :EnumValue;
    backgroundId @3 :VisualId;
    groundId @4 :VisualId;
    dualMode @5 :Bool;
    twoPlayer @6 :Bool;
    flipGravity @7 :Bool;
    songOffset @8 :Timestamp;
    guidelines @9 :Guidelines;
    songFadeIn @10 :Bool;
    songFadeOut @11 :Bool;
    groundLineId @12 :VisualId;
    fontId @13 :VisualId;
    platformerMode @14 :Bool;
    colorChannels @15 :List(ColorChannel);
}
