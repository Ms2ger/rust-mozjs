/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::{JSContext};

enum ThingRootKind {
    THING_ROOT_OBJECT = 0,
    THING_ROOT_SHAPE,
    THING_ROOT_BASE_SHAPE,
    THING_ROOT_TYPE_OBJECT,
    THING_ROOT_STRING,
    THING_ROOT_ION_CODE,
    THING_ROOT_SCRIPT,
    THING_ROOT_ID,
    THING_ROOT_PROPERTY_ID,
    THING_ROOT_VALUE,
    THING_ROOT_TYPE,
    THING_ROOT_BINDINGS,
    THING_ROOT_PROPERTY_DESCRIPTOR,
    THING_ROOT_LIMIT
}

pub trait Rootable {
    fn kind() -> ThingRootKind;
    fn poisoned(self) -> bool;
}

pub struct Rooted<T: Rootable> {
//#ifdef JSGC_TRACK_EXACT_ROOTS
    priv stack: **Rooted<*()>,
    priv prev: *Rooted<*()>,
//#endif

//#ifdef JSGC_ROOT_ANALYSIS
    priv scanned: bool,
//#endif

    /*
     * |ptr| must be the last field in Rooted because the analysis treats all
     * Rooted as Rooted<void*> during the analysis. See bug 829372.
     */
    priv ptr: T,
}

struct Context {
    _runtime: *(),
    _compartment: *(),
    _zone: *(),

//#ifdef JSGC_TRACK_EXACT_ROOTS
    /*
     * Stack allocated GC roots for stack GC heap pointers, which may be
     * overwritten if moved during a GC.
     */
    thingGCRooters: *[Rooted<*()>, ..THING_ROOT_LIMIT as uint],
//#endif
}

impl<T> Rooted<T> {
    priv fn init(&self, cx: &Context) {
//#ifdef JSGC_TRACK_EXACT_ROOTS
        //let kind = T::kind();
        //self.stack = &cx.thingGCRooters[kind];
        //self.prev = *self.stack;
        // *stack = reinterpret_cast<Rooted<void*>*>(this);

        assert!(!self.ptr.poisoned());
//#endif
    }
}
/*
  public:
    pub fn new(cx: *JSContext) {
      : ptr(js::GCMethods<T>::initial())
    {
        MOZ_GUARD_OBJECT_NOTIFIER_INIT;
        MOZ_ASSERT(js::IsInRequest(cx));
        init(js::ContextFriendFields::get(cx));
    }

    Rooted(JSContext *cx, T initial
           MOZ_GUARD_OBJECT_NOTIFIER_PARAM)
      : ptr(initial)
    {
        MOZ_GUARD_OBJECT_NOTIFIER_INIT;
        MOZ_ASSERT(js::IsInRequest(cx));
        init(js::ContextFriendFields::get(cx));
    }

    Rooted(js::ContextFriendFields *cx
           MOZ_GUARD_OBJECT_NOTIFIER_PARAM)
      : ptr(js::GCMethods<T>::initial())
    {
        MOZ_GUARD_OBJECT_NOTIFIER_INIT;
        init(cx);
    }

    Rooted(js::ContextFriendFields *cx, T initial
           MOZ_GUARD_OBJECT_NOTIFIER_PARAM)
      : ptr(initial)
    {
        MOZ_GUARD_OBJECT_NOTIFIER_INIT;
        init(cx);
    }

    Rooted(js::PerThreadDataFriendFields *pt
           MOZ_GUARD_OBJECT_NOTIFIER_PARAM)
      : ptr(js::GCMethods<T>::initial())
    {
        MOZ_GUARD_OBJECT_NOTIFIER_INIT;
        init(pt);
    }

    Rooted(js::PerThreadDataFriendFields *pt, T initial
           MOZ_GUARD_OBJECT_NOTIFIER_PARAM)
      : ptr(initial)
    {
        MOZ_GUARD_OBJECT_NOTIFIER_INIT;
        init(pt);
    }

    Rooted(JSRuntime *rt
           MOZ_GUARD_OBJECT_NOTIFIER_PARAM)
      : ptr(js::GCMethods<T>::initial())
    {
        MOZ_GUARD_OBJECT_NOTIFIER_INIT;
        init(js::PerThreadDataFriendFields::getMainThread(rt));
    }

    Rooted(JSRuntime *rt, T initial
           MOZ_GUARD_OBJECT_NOTIFIER_PARAM)
      : ptr(initial)
    {
        MOZ_GUARD_OBJECT_NOTIFIER_INIT;
        init(js::PerThreadDataFriendFields::getMainThread(rt));
    }

    // Note that we need to let the compiler generate the default destructor in
    // non-exact-rooting builds because of a bug in the instrumented PGO builds
    // using MSVC, see bug 915735 for more details.
#ifdef JSGC_TRACK_EXACT_ROOTS
    ~Rooted() {
        JS_ASSERT(*stack == reinterpret_cast<Rooted<void*>*>(this));
        *stack = prev;
    }
#endif

#ifdef JSGC_TRACK_EXACT_ROOTS
    Rooted<T> *previous() { return prev; }
#endif

*/    /*
     * Important: Return a reference here so passing a Rooted<T> to
     * something that takes a |const T&| is not a GC hazard.
     */
/*    operator const T&() const { return ptr; }
    T operator->() const { return ptr; }
    T *address() { return &ptr; }
    const T *address() const { return &ptr; }
    T &get() { return ptr; }
    const T &get() const { return ptr; }

    T &operator=(T value) {
        JS_ASSERT(!js::GCMethods<T>::poisoned(value));
        ptr = value;
        return ptr;
    }

    T &operator=(const Rooted &value) {
        ptr = value;
        return ptr;
    }

    void set(T value) {
        JS_ASSERT(!js::GCMethods<T>::poisoned(value));
        ptr = value;
    }

    bool operator!=(const T &other) const { return ptr != other; }
    bool operator==(const T &other) const { return ptr == other; }

  private:
#if defined(DEBUG) && defined(JS_GC_ZEAL) && defined(JSGC_ROOT_ANALYSIS) && !defined(JS_THREADSAFE)
*/    /* Has the rooting analysis ever scanned this Rooted's stack location? */
/*    friend void JS::CheckStackRoots(JSContext*);
#endif

    MOZ_DECL_USE_GUARD_OBJECT_NOTIFIER

    Rooted(const Rooted &) MOZ_DELETE;
};
*/
