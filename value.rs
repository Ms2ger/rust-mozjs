/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::{JSVal, JSObject};

#[deriving(Eq,Clone)]
pub struct Value {
    priv v: u64
}

static JSVAL_TAG_MAX_DOUBLE: u64 = 0x1FFF0;

//static JSVAL_TYPE_DOUBLE: u64 = 0x00;
static JSVAL_TYPE_INT32: u64 = 0x01;
static JSVAL_TYPE_UNDEFINED: u64 = 0x02;
static JSVAL_TYPE_BOOLEAN: u64 = 0x03;
static JSVAL_TYPE_MAGIC: u64 = 0x04;
static JSVAL_TYPE_STRING: u64 = 0x05;
static JSVAL_TYPE_NULL: u64 = 0x06;
static JSVAL_TYPE_OBJECT: u64 = 0x07;
//static JSVAL_TYPE_UNKNOWN: u64 = 0x20;

static JSVAL_TAG_SHIFT: int = 47;

static JSVAL_PAYLOAD_MASK: u64 = 0x00007FFFFFFFFFFF;

#[repr(u32)]
enum ValueTag {
    JSVAL_TAG_INT32     = JSVAL_TAG_MAX_DOUBLE | JSVAL_TYPE_INT32,
    JSVAL_TAG_UNDEFINED = JSVAL_TAG_MAX_DOUBLE | JSVAL_TYPE_UNDEFINED,
    JSVAL_TAG_STRING    = JSVAL_TAG_MAX_DOUBLE | JSVAL_TYPE_STRING,
    JSVAL_TAG_BOOLEAN   = JSVAL_TAG_MAX_DOUBLE | JSVAL_TYPE_BOOLEAN,
    JSVAL_TAG_MAGIC     = JSVAL_TAG_MAX_DOUBLE | JSVAL_TYPE_MAGIC,
    JSVAL_TAG_NULL      = JSVAL_TAG_MAX_DOUBLE | JSVAL_TYPE_NULL,
    JSVAL_TAG_OBJECT    = JSVAL_TAG_MAX_DOUBLE | JSVAL_TYPE_OBJECT,
}

#[repr(u64)]
enum ValueShiftedTag {
    JSVAL_SHIFTED_TAG_MAX_DOUBLE   = (((JSVAL_TAG_MAX_DOUBLE as u64) << JSVAL_TAG_SHIFT) | 0xFFFFFFFFu64),
    JSVAL_SHIFTED_TAG_INT32        = ((JSVAL_TAG_INT32 as u64)      << JSVAL_TAG_SHIFT),
    JSVAL_SHIFTED_TAG_UNDEFINED    = ((JSVAL_TAG_UNDEFINED as u64)  << JSVAL_TAG_SHIFT),
    JSVAL_SHIFTED_TAG_STRING       = ((JSVAL_TAG_STRING as u64)     << JSVAL_TAG_SHIFT),
    JSVAL_SHIFTED_TAG_BOOLEAN      = ((JSVAL_TAG_BOOLEAN as u64)    << JSVAL_TAG_SHIFT),
    JSVAL_SHIFTED_TAG_MAGIC        = ((JSVAL_TAG_MAGIC as u64)      << JSVAL_TAG_SHIFT),
    JSVAL_SHIFTED_TAG_NULL         = ((JSVAL_TAG_NULL as u64)       << JSVAL_TAG_SHIFT),
    JSVAL_SHIFTED_TAG_OBJECT       = ((JSVAL_TAG_OBJECT as u64)     << JSVAL_TAG_SHIFT)
}

impl Value {
    #[inline(always)]
    fn new(tag: ValueTag, payload: u64) -> Value {
        Value {
            v: ((tag as u64) << JSVAL_TAG_SHIFT) | payload
        }
    }

    pub fn from_jsval(val: JSVal) -> Value {
        Value {
            v: val.v
        }
    }

    pub fn to_jsval(&self) -> JSVal {
        JSVal {
            v: self.v
        }
    }
}

impl Value {
    pub fn is_null(&self) -> bool {
        false
    }

    pub fn is_primitive(&self) -> bool {
        self.v < (JSVAL_SHIFTED_TAG_OBJECT as u64)
    }

    pub fn is_object_or_null(&self) -> bool {
        static JSVAL_LOWER_INCL_SHIFTED_TAG_OF_OBJ_OR_NULL_SET: u64 =
            JSVAL_SHIFTED_TAG_NULL as u64;
        assert!((self.v >> JSVAL_TAG_SHIFT) <= (JSVAL_TAG_OBJECT as u64));
        self.v >= JSVAL_LOWER_INCL_SHIFTED_TAG_OF_OBJ_OR_NULL_SET
    }

    pub fn to_object_or_null(&self) -> *JSObject {
        assert!(self.is_object_or_null());
        let ptrBits = self.v & JSVAL_PAYLOAD_MASK;
        assert!((ptrBits & 0x7) == 0);
        ptrBits as uint as *JSObject
    }
}

#[inline(always)]
pub fn NullValue() -> Value {
    Value::new(JSVAL_TAG_NULL, 0)
}

#[inline(always)]
pub fn UndefinedValue() -> Value {
    Value::new(JSVAL_TAG_UNDEFINED, 0)
}

#[inline(always)]
pub fn BooleanValue(b: bool) -> Value {
    Value::new(JSVAL_TAG_BOOLEAN, b as u64)
}

#[inline(always)]
pub fn Int32Value(i: i32) -> Value {
    Value::new(JSVAL_TAG_INT32, i as u64)
}
/*
#[inline(always)]
pub fn JSVAL_TO_OBJECT(v: Value) -> *JSObject {
    let bits = (v.v & JSVAL_PAYLOAD_MASK);
    assert!(bits & 0x7 == 0);
    bits as *JSObject
}

#[inline(always)]
pub fn JSVAL_IS_PRIMITIVE(v: Value) -> bool {
    v.v < JSVAL_SHIFTED_TAG_OBJECT
}

#[inline(always)]
pub fn JSVAL_IS_OBJECT(v: Value) -> bool {
    v.v >= JSVAL_SHIFTED_TAG_OBJECT
}

#[inline(always)]
pub fn JSVAL_TO_PRIVATE(v: Value) -> *() {
    assert!(v.v & 0x8000000000000000 == 0);
    (v.v << 1) as *()
}
*/
