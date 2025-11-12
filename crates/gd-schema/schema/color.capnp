@0xcabdde5e8c86b3ab;

struct OpaqueColor {
    r @0 :UInt8;
    g @1 :UInt8;
    b @2 :UInt8;
}

using Color = OpaqueColor;

struct AlphaColor {
    r @0 :UInt8;
    g @1 :UInt8;
    b @2 :UInt8;
    a @3 :UInt8;
}
