use jsapi::*;
use std::os::raw::{c_char, c_void};

pub enum Action { }
unsafe impl Sync for ProxyTraps { }
/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JobQueueTraps {
    pub getIncumbentGlobal: ::std::option::Option<
            unsafe extern "C" fn(queue: *const c_void, cx: *mut JSContext) -> *mut JSObject>,
    pub enqueuePromiseJob: ::std::option::Option<
            unsafe extern "C" fn(queue: *const c_void, cx: *mut JSContext, promise: HandleObject,
                                 job: HandleObject, allocationSite: HandleObject,
                                 incumbentGlobal: HandleObject) -> bool>,
    pub empty: ::std::option::Option<unsafe extern "C" fn(queue: *const c_void) -> bool>,
}
impl ::std::default::Default for JobQueueTraps {
    fn default() -> JobQueueTraps { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProxyTraps {
    pub enter: ::std::option::Option<unsafe extern "C" fn
                                         (cx: *mut JSContext,
                                          proxy: HandleObject, id: HandleId,
                                          action: Action, bp: *mut bool) -> bool>,
    pub getOwnPropertyDescriptor: ::std::option::Option<unsafe extern "C" fn
                                                            (cx:
                                                                 *mut JSContext,
                                                             proxy:
                                                                 HandleObject,
                                                             id: HandleId,
                                                             desc:
                                                                 MutableHandle<PropertyDescriptor>)
                                                            -> bool>,
    pub defineProperty: ::std::option::Option<unsafe extern "C" fn
                                                  (cx: *mut JSContext,
                                                   proxy: HandleObject,
                                                   id: HandleId,
                                                   desc:
                                                       Handle<PropertyDescriptor>,
                                                   result:
                                                       *mut ObjectOpResult)
                                                  -> bool>,
    pub ownPropertyKeys: ::std::option::Option<unsafe extern "C" fn
                                                   (cx: *mut JSContext,
                                                    proxy: HandleObject,
                                                    props: *mut AutoIdVector)
                                                   -> bool>,
    pub delete_: ::std::option::Option<unsafe extern "C" fn
                                           (cx: *mut JSContext,
                                            proxy: HandleObject, id: HandleId,
                                            result: *mut ObjectOpResult)
                                           -> bool>,
    pub enumerate: ::std::option::Option<unsafe extern "C" fn
                                             (cx: *mut JSContext,
                                              proxy: HandleObject,
                                              props: *mut AutoIdVector)
                                             -> bool>,
    pub getPrototypeIfOrdinary: ::std::option::Option<unsafe extern "C" fn
                                                          (cx: *mut JSContext,
                                                           proxy: HandleObject,
                                                           isOrdinary: *mut bool,
                                                           protop: MutableHandleObject)
                                                          -> bool>,
    pub preventExtensions: ::std::option::Option<unsafe extern "C" fn
                                                     (cx: *mut JSContext,
                                                      proxy: HandleObject,
                                                      result:
                                                          *mut ObjectOpResult)
                                                     -> bool>,
    pub isExtensible: ::std::option::Option<unsafe extern "C" fn
                                                (cx: *mut JSContext,
                                                 proxy: HandleObject,
                                                 succeeded: *mut bool) -> bool>,
    pub has: ::std::option::Option<unsafe extern "C" fn
                                       (cx: *mut JSContext,
                                        proxy: HandleObject, id: HandleId,
                                        bp: *mut bool) -> bool>,
    pub get: ::std::option::Option<unsafe extern "C" fn
                                       (cx: *mut JSContext,
                                        proxy: HandleObject,
                                        receiver: HandleValue, id: HandleId,
                                        vp: MutableHandleValue) -> bool>,
    pub set: ::std::option::Option<unsafe extern "C" fn
                                       (cx: *mut JSContext,
                                        proxy: HandleObject,
                                        id: HandleId, v: HandleValue,
                                        receiver: HandleValue,
                                        result: *mut ObjectOpResult) -> bool>,
    pub call: ::std::option::Option<unsafe extern "C" fn
                                        (cx: *mut JSContext,
                                         proxy: HandleObject,
                                         args: *const CallArgs) -> bool>,
    pub construct: ::std::option::Option<unsafe extern "C" fn
                                             (cx: *mut JSContext,
                                              proxy: HandleObject,
                                              args: *const CallArgs) -> bool>,
    pub hasOwn: ::std::option::Option<unsafe extern "C" fn
                                          (cx: *mut JSContext,
                                           proxy: HandleObject, id: HandleId,
                                           bp: *mut bool) -> bool>,
    pub getOwnEnumerablePropertyKeys: ::std::option::Option<unsafe extern "C" fn
                                                                (cx:
                                                                     *mut JSContext,
                                                                 proxy:
                                                                     HandleObject,
                                                                 props:
                                                                     *mut AutoIdVector)
                                                                -> bool>,
    pub nativeCall: ::std::option::Option<unsafe extern "C" fn
                                              (cx: *mut JSContext,
                                               test: IsAcceptableThis,
                                               _impl: NativeImpl,
                                               args: CallArgs) -> bool>,
    pub hasInstance: ::std::option::Option<unsafe extern "C" fn
                                               (cx: *mut JSContext,
                                                proxy: HandleObject,
                                                v: MutableHandleValue,
                                                bp: *mut bool) -> bool>,
    pub objectClassIs: ::std::option::Option<unsafe extern "C" fn
                                                 (obj: HandleObject,
                                                  classValue: ESClass,
                                                  cx: *mut JSContext) -> bool>,
    pub className: ::std::option::Option<unsafe extern "C" fn
                                             (cx: *mut JSContext,
                                              proxy: HandleObject)
                                             -> *const i8>,
    pub fun_toString: ::std::option::Option<unsafe extern "C" fn
                                                (cx: *mut JSContext,
                                                 proxy: HandleObject,
                                                 isToString: bool)
                                                -> *mut JSString>,
    pub boxedValue_unbox: ::std::option::Option<unsafe extern "C" fn
                                                    (cx: *mut JSContext,
                                                     proxy: HandleObject,
                                                     vp: MutableHandleValue)
                                                    -> bool>,
    pub defaultValue: ::std::option::Option<unsafe extern "C" fn
                                                (cx: *mut JSContext,
                                                 obj: HandleObject,
                                                 hint: JSType,
                                                 vp: MutableHandleValue)
                                                -> bool>,
    pub trace: ::std::option::Option<unsafe extern "C" fn
                                         (trc: *mut JSTracer,
                                          proxy: *mut JSObject)>,
    pub finalize: ::std::option::Option<unsafe extern "C" fn
                                            (fop: *mut JSFreeOp,
                                             proxy: *mut JSObject)>,
    pub objectMoved: ::std::option::Option<unsafe extern "C" fn
                                               (proxy: *mut JSObject,
                                                old: *mut JSObject)
					       -> usize>,
    pub isCallable: ::std::option::Option<unsafe extern "C" fn
                                              (obj: *mut JSObject) -> bool>,
    pub isConstructor: ::std::option::Option<unsafe extern "C" fn
                                                 (obj: *mut JSObject) -> bool>,
}
impl ::std::default::Default for ProxyTraps {
    fn default() -> ProxyTraps { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapperProxyHandler {
    pub mTraps: ProxyTraps,
}
impl ::std::default::Default for WrapperProxyHandler {
    fn default() -> WrapperProxyHandler { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ForwardingProxyHandler {
    pub mTraps: ProxyTraps,
    pub mExtra: *const ::libc::c_void,
}
impl ::std::default::Default for ForwardingProxyHandler {
    fn default() -> ForwardingProxyHandler { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn InvokeGetOwnPropertyDescriptor(handler: *const ::libc::c_void,
                                          cx: *mut JSContext,
                                          proxy: HandleObject, id: HandleId,
                                          desc:
                                              MutableHandle<PropertyDescriptor>)
     -> bool;
    pub fn InvokeHasOwn(handler: *const ::libc::c_void,
                        cx: *mut JSContext, proxy: HandleObject,
                        id: HandleId, bp: *mut bool) -> bool;
    pub fn RUST_JS_NumberValue(d: f64, dest: *mut JS::Value);
    pub fn RUST_FUNCTION_VALUE_TO_JITINFO(v: Value) -> *const JSJitInfo;
    pub fn CreateCallArgsFromVp(argc: u32, v: *mut Value) -> CallArgs;
    pub fn CallJitGetterOp(info: *const JSJitInfo, cx: *mut JSContext,
                           thisObj: HandleObject,
                           specializedThis: *mut ::libc::c_void, argc: u32,
                           vp: *mut Value) -> bool;
    pub fn CallJitSetterOp(info: *const JSJitInfo, cx: *mut JSContext,
                           thisObj: HandleObject,
                           specializedThis: *mut ::libc::c_void, argc: u32,
                           vp: *mut Value) -> bool;
    pub fn CallJitMethodOp(info: *const JSJitInfo, cx: *mut JSContext,
                           thisObj: HandleObject,
                           specializedThis: *mut ::libc::c_void, argc: u32,
                           vp: *mut Value) -> bool;
    pub fn CreateProxyHandler(aTraps: *const ProxyTraps,
                              aExtra: *const ::libc::c_void)
     -> *const ::libc::c_void;
    pub fn CreateWrapperProxyHandler(aTraps: *const ProxyTraps)
     -> *const ::libc::c_void;
    pub fn CreateRustJSPrincipal(origin: *const ::libc::c_void,
                                  destroy: Option<unsafe extern "C" fn
                                                  (principal: *mut JSPrincipals)>,
                                  write: Option<unsafe extern "C" fn
                                                (cx: *mut JSContext,
                                                 writer: *mut JSStructuredCloneWriter)
                                                 -> bool>)
     -> *mut JSPrincipals;
    pub fn GetPrincipalOrigin(principal: *const JSPrincipals)
     -> *const ::libc::c_void;
    pub fn GetCrossCompartmentWrapper() -> *const ::libc::c_void;
    pub fn GetSecurityWrapper() -> *const ::libc::c_void;
    pub fn NewCompileOptions(aCx: *mut JSContext, aFile: *const ::libc::c_char,
                             aLine: u32) -> *mut ReadOnlyCompileOptions;
    pub fn DeleteCompileOptions(aOpts: *mut ReadOnlyCompileOptions);
    pub fn NewProxyObject(aCx: *mut JSContext,
                          aHandler: *const ::libc::c_void, aPriv: HandleValue,
                          proto: *mut JSObject)
     -> *mut JSObject;
    pub fn WrapperNew(aCx: *mut JSContext, aObj: HandleObject,
                      aHandler: *const ::libc::c_void, aClass: *const JSClass,
                      aSingleton: bool)
                      -> *mut JSObject;



    pub fn NewWindowProxy(aCx: *mut JSContext, aObj: HandleObject,
                          aHandler: *const ::libc::c_void)
     -> *mut JSObject;
    pub fn GetWindowProxyClass() -> *const Class;
    pub fn GetProxyReservedSlot(obj: *mut JSObject, slot: u32, dest: *mut JS::Value);
    pub fn GetProxyPrivate(obj: *mut JSObject, dest: *mut JS::Value);
    pub fn SetProxyReservedSlot(obj: *mut JSObject, slot: u32, val: *const JS::Value);
    pub fn SetProxyPrivate(obj: *mut JSObject, expando: *const JS::Value);

    pub fn RUST_JSID_IS_INT(id: HandleId) -> bool;
    pub fn RUST_JSID_TO_INT(id: HandleId) -> i32;
    pub fn int_to_jsid(i: i32, id: MutableHandleId);
    pub fn RUST_JSID_IS_STRING(id: HandleId) -> bool;
    pub fn RUST_JSID_TO_STRING(id: HandleId) -> *mut JSString;
    pub fn RUST_SYMBOL_TO_JSID(sym: *mut Symbol, id: MutableHandleId);
    pub fn SetBuildId(buildId: *mut JS::BuildIdCharVector, chars: *const u8, len: usize) -> bool;
    pub fn RUST_SET_JITINFO(func: *mut JSFunction, info: *const JSJitInfo);
    pub fn RUST_INTERNED_STRING_TO_JSID(cx: *mut JSContext,
                                        str: *mut JSString,
                                        id: MutableHandleId);
    pub fn RUST_js_GetErrorMessage(userRef: *mut ::libc::c_void,
                                   errorNumber: u32)
     -> *const JSErrorFormatString;
    pub fn IsProxyHandlerFamily(obj: *mut JSObject) -> u8;
    pub fn GetProxyHandlerExtra(obj: *mut JSObject) -> *const ::libc::c_void;
    pub fn GetProxyHandler(obj: *mut JSObject) -> *const ::libc::c_void;
    pub fn ReportError(aCx: *mut JSContext, aError: *const i8);
    pub fn IsWrapper(obj: *mut JSObject) -> bool;
    pub fn UnwrapObjectStatic(obj: *mut JSObject) -> *mut JSObject;
    pub fn UnwrapObjectDynamic(obj: *mut JSObject, cx: *mut JSContext, stopAtOuter: u8) -> *mut JSObject;
    pub fn UncheckedUnwrapObject(obj: *mut JSObject, stopAtOuter: u8) -> *mut JSObject;
    pub fn CreateAutoIdVector(cx: *mut JSContext) -> *mut AutoIdVector;
    pub fn AppendToAutoIdVector(v: *mut AutoIdVector, id: HandleId) -> bool;
    pub fn SliceAutoIdVector(v: *const AutoIdVector, length: *mut usize) -> *const jsid;
    pub fn DestroyAutoIdVector(v: *mut AutoIdVector);
    pub fn CreateAutoObjectVector(aCx: *mut JSContext)
     -> *mut AutoObjectVector;
    pub fn AppendToAutoObjectVector(v: *mut AutoObjectVector,
                                    obj: *mut JSObject) -> bool;
    pub fn DeleteAutoObjectVector(v: *mut AutoObjectVector);
    pub fn CollectServoSizes(cx: *mut JSContext, sizes: *mut ServoSizes, get_size: Option<unsafe extern "C" fn (obj: *mut JSObject) -> usize>) -> bool;
    pub fn InitializeMemoryReporter(want_to_measure: Option<unsafe extern "C" fn (obj: *mut JSObject) -> bool>);
    pub fn CallIdTracer(trc: *mut JSTracer, idp: *mut Heap<jsid>,
                        name: *const ::libc::c_char);
    pub fn CallValueTracer(trc: *mut JSTracer, valuep: *mut Heap<Value>,
                           name: *const ::libc::c_char);
    pub fn CallObjectTracer(trc: *mut JSTracer,
                            objp: *mut Heap<*mut JSObject>,
                            name: *const ::libc::c_char);
    pub fn CallStringTracer(trc: *mut JSTracer,
                            strp: *mut Heap<*mut JSString>,
                            name: *const ::libc::c_char);
    pub fn CallScriptTracer(trc: *mut JSTracer,
                            scriptp: *mut Heap<*mut JSScript>,
                            name: *const ::libc::c_char);
    pub fn CallFunctionTracer(trc: *mut JSTracer,
                              funp: *mut Heap<*mut JSFunction>,
                              name: *const ::libc::c_char);
    pub fn CallUnbarrieredObjectTracer(trc: *mut JSTracer,
                                       objp: *mut *mut JSObject,
                                       name: *const ::libc::c_char);
    pub fn CallObjectRootTracer(trc: *mut JSTracer,
                                objp: *mut *mut JSObject,
                                name: *const ::libc::c_char);
    pub fn CallValueRootTracer(trc: *mut JSTracer,
                                valp: *mut Value,
                                name: *const ::libc::c_char);
    pub fn GetProxyHandlerFamily() -> *const c_void;

    pub fn GetInt8ArrayLengthAndData(obj: *mut JSObject,
                                     length: *mut u32,
                                     isSharedMemory: *mut bool,
                                     data: *mut *mut i8);
    pub fn GetUint8ArrayLengthAndData(obj: *mut JSObject,
                                      length: *mut u32,
                                      isSharedMemory: *mut bool,
                                      data: *mut *mut u8);
    pub fn GetUint8ClampedArrayLengthAndData(obj: *mut JSObject,
                                             length: *mut u32,
                                             isSharedMemory: *mut bool,
                                             data: *mut *mut u8);
    pub fn GetInt16ArrayLengthAndData(obj: *mut JSObject,
                                      length: *mut u32,
                                      isSharedMemory: *mut bool,
                                      data: *mut *mut i16);
    pub fn GetUint16ArrayLengthAndData(obj: *mut JSObject,
                                       length: *mut u32,
                                       isSharedMemory: *mut bool,
                                       data: *mut *mut u16);
    pub fn GetInt32ArrayLengthAndData(obj: *mut JSObject,
                                      length: *mut u32,
                                      isSharedMemory: *mut bool,
                                      data: *mut *mut i32);
    pub fn GetUint32ArrayLengthAndData(obj: *mut JSObject,
                                       length: *mut u32,
                                       isSharedMemory: *mut bool,
                                       data: *mut *mut u32);
    pub fn GetFloat32ArrayLengthAndData(obj: *mut JSObject,
                                        length: *mut u32,
                                        isSharedMemory: *mut bool,
                                        data: *mut *mut f32);
    pub fn GetFloat64ArrayLengthAndData(obj: *mut JSObject,
                                        length: *mut u32,
                                        isSharedMemory: *mut bool,
                                        data: *mut *mut f64);

    pub fn NewJSAutoStructuredCloneBuffer(scope: JS::StructuredCloneScope,
                                          callbacks: *const JSStructuredCloneCallbacks) ->
                                         *mut JSAutoStructuredCloneBuffer;
    pub fn DeleteJSAutoStructuredCloneBuffer(buf: *mut JSAutoStructuredCloneBuffer);
    pub fn GetLengthOfJSStructuredCloneData(data: *mut JSStructuredCloneData) -> usize;
    pub fn CopyJSStructuredCloneData(src: *const JSStructuredCloneData, dest: *mut u8);
    pub fn WriteBytesToJSStructuredCloneData(src: *const u8, len: usize, dest: *mut JSStructuredCloneData);
    pub fn JS_ComputeThis (cx: *mut JSContext , vp: *mut JS::Value, dest: *mut JS::Value);
    pub fn JS_GetModuleHostDefinedField (module: *mut JSObject, dest: *mut JS::Value);
    pub fn JS_GetPromiseResult (promise: JS::HandleObject, dest: JS::MutableHandleValue);
    pub fn JS_THIS (cx: *mut JSContext , vp: *mut JS::Value, dest: *mut JS::Value);
    pub fn JS_GetNaNValue (cx: *mut JSContext, dest: *mut JS::Value);
    pub fn JS_GetPositiveInfinityValue (cx: *mut JSContext, dest: *mut JS::Value);
    pub fn JS_GetEmptyStringValue (cx: *mut JSContext, dest: *mut JS::Value);
    pub fn JS_GetReservedSlot (obj: *mut JSObject , index: u32, dest: *mut JS::Value);
    pub fn EncodeStringToUTF8(cx: *mut JSContext, str: JS::HandleString, cb: fn(*const c_char));
    pub fn CreateJobQueue(traps: *const JobQueueTraps, queue: *const c_void) -> *mut JS::JobQueue;
    pub fn DeleteJobQueue(queue: *mut JS::JobQueue);
    pub fn DispatchableRun(cx: *mut JSContext, ptr: *mut JS::Dispatchable, mb: JS::Dispatchable_MaybeShuttingDown);
    pub fn StreamConsumerConsumeChunk(sc: *mut JS::StreamConsumer, begin: *const u8, length: usize) -> bool;
    pub fn StreamConsumerStreamEnd(cx: *mut JS::StreamConsumer);
    pub fn StreamConsumerStreamError(cx: *mut JS::StreamConsumer, errorCode: usize);
    pub fn StreamConsumerNoteResponseURLs(sc: *mut JS::StreamConsumer, maybeUrl: *const c_char, maybeSourceMapUrl: *const c_char);
}
