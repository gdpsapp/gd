@0xeb50502d690763c0;

struct Option(T) {
    union {
        value @0 :T;
        empty @1 :Void;
    }
}

# define specializations for primitives

struct OptionBool {
    union {
        value @0 :Bool;
        empty @1 :Void;
    }
}

struct OptionInt8 {
    union {
        value @0 :Int8;
        empty @1 :Void;
    }
}

struct OptionInt16 {
    union {
        value @0 :Int16;
        empty @1 :Void;
    }
}

struct OptionInt32 {
    union {
        value @0 :Int32;
        empty @1 :Void;
    }
}

struct OptionInt64 {
    union {
        value @0 :Int64;
        empty @1 :Void;
    }
}

struct OptionUInt8 {
    union {
        value @0 :UInt8;
        empty @1 :Void;
    }
}

struct OptionUInt16 {
    union {
        value @0 :UInt16;
        empty @1 :Void;
    }
}

struct OptionUInt32 {
    union {
        value @0 :UInt32;
        empty @1 :Void;
    }
}

struct OptionUInt64 {
    union {
        value @0 :UInt64;
        empty @1 :Void;
    }
}

struct OptionFloat32 {
    union {
        value @0 :Float32;
        empty @1 :Void;
    }
}

struct OptionFloat64 {
    union {
        value @0 :Float64;
        empty @1 :Void;
    }
}
