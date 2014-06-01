/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

#![doc = "Rust wrappers around the raw JS apis"]

use libc::types::os::arch::c95::{size_t, c_uint};
use libc::c_char;
use std::cmp;
use std::rc;
use std::rt::Runtime;
use jsapi::*;
use jsval::{JSVal, NullValue};
use default_stacksize;
use default_heapsize;
use JSOPTION_VAROBJFIX;
use JSOPTION_METHODJIT;
use JSOPTION_TYPE_INFERENCE;
use ERR;
use std::str::raw::from_c_str;
use green::task::GreenTask;

// ___________________________________________________________________________
// friendly Rustic API to runtimes

pub type runtime = rc::Rc<rt_rsrc>;

pub struct rt_rsrc {
    runtime: *mut JSRuntime,
    context: *mut JSContext,
}

impl Drop for rt_rsrc {
    fn drop(&mut self) {
        unsafe {
            JS_DestroyContext(self.context);
            JS_Finish(self.runtime);
        }
    }
}

extern fn gc_callback(rt: *mut JSRuntime, _status: JSGCStatus) {
    use std::rt::local::Local;
    use std::rt::task::Task;
    unsafe {
        let mut task = Local::borrow(None::<Task>);
        let green_task: Box<GreenTask> = task.maybe_take_runtime().unwrap();
        let (start, end) = green_task.stack_bounds();
        JS_SetNativeStackBounds(rt, cmp::min(start, end), cmp::max(start, end));
        task.put_runtime(green_task);
    }
}

pub fn runtime(wrap_for_same_compartment: JSSameCompartmentWrapObjectCallback,
               pre_wrap: JSPreWrapCallback) -> runtime {
    unsafe {
        let runtime = JS_Init(default_heapsize);
        assert!(runtime.is_not_null());

        JS_SetGCCallback(runtime, Some(gc_callback));

        // JS_SetWrapObjectCallbacks clobbers the existing wrap callback,
        // and JSCompartment::wrap crashes if that happens. The only way
        // to retrieve the default callback is as the result of
        // JS_SetWrapObjectCallbacks, which is why we call it twice.
        let callback = JS_SetWrapObjectCallbacks(runtime,
                                                 None,
                                                 wrap_for_same_compartment,
                                                 None);
        JS_SetWrapObjectCallbacks(runtime,
                                  callback,
                                  wrap_for_same_compartment,
                                  pre_wrap);

        let context = JS_NewContext(runtime, default_stacksize as size_t);
        assert!(context.is_not_null());

        JS_SetOptions(context, JSOPTION_VAROBJFIX | JSOPTION_METHODJIT |
                               JSOPTION_TYPE_INFERENCE);
        JS_SetVersion(context, JSVERSION_LATEST);
        JS_SetErrorReporter(context, Some(reportError));
        JS_SetGCZeal(context, 0, JS_DEFAULT_ZEAL_FREQ);

        rc::Rc::new(rt_rsrc {
            runtime: runtime,
            context: context,
        })
    }
}

impl rt_rsrc {
    pub fn context(&self) -> *mut JSContext {
        self.context
    }

    pub fn runtime(&self) -> *mut JSRuntime {
        self.runtime
    }

    pub fn evaluate_script(&self, glob: *mut JSObject, script: ~str, filename: ~str, line_num: uint)
                    -> Result<(),()> {
        let script_utf16 = script.to_utf16();
        filename.to_c_str().with_ref(|filename_cstr| {
            let mut rval: JSVal = NullValue();
            debug!("Evaluating script from {:s} with content {}", filename, script);
            unsafe {
                if ERR == JS_EvaluateUCScript(self.context, glob,
                                              script_utf16.as_ptr(), script_utf16.len() as c_uint,
                                              filename_cstr, line_num as c_uint,
                                              &mut rval) {
                    debug!("...err!");
                    Err(())
                } else {
                    // we could return the script result but then we'd have
                    // to root it and so forth and, really, who cares?
                    debug!("...ok!");
                    Ok(())
                }
            }
        })
    }
}

pub extern fn reportError(_cx: *mut JSContext, msg: *c_char, report: *mut JSErrorReport) {
    unsafe {
        let fnptr = (*report).filename;
        let fname = if fnptr.is_not_null() {from_c_str(fnptr)} else {"none".to_owned()};
        let lineno = (*report).lineno;
        let msg = from_c_str(msg);
        error!("Error at {:s}:{}: {:s}\n", fname, lineno, msg);
    }
}

pub fn with_compartment<R>(cx: *mut JSContext, object: *mut JSObject, cb: || -> R) -> R {
    unsafe {
        let call = JS_EnterCrossCompartmentCall(cx, object);
        let result = cb();
        JS_LeaveCrossCompartmentCall(call);
        result
    }
}
