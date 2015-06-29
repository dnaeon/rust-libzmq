extern crate libc;

//
// 0MQ errors
//

// A number random enough not to collide with different
// errno ranges on different OSes. The assumption is that
// error_t is at least 32-bit type.
const ZMQ_HAUSNUMERO: usize = 156384712;

// Native 0MQ error codes
pub const EFSM: usize = ZMQ_HAUSNUMERO + 51;
pub const ENOCOMPATPROTO: usize = ZMQ_HAUSNUMERO + 52;
pub const ETERM: usize = ZMQ_HAUSNUMERO + 53;
pub const EMTHREAD: usize = ZMQ_HAUSNUMERO + 54;

//
// 0MQ infrastructure (a.k.a. context) initialisation & termination
//

// Context options
pub const ZMQ_IO_THREADS: usize = 1;
pub const ZMQ_MAX_SOCKETS: usize = 2;
pub const ZMQ_SOCKET_LIMIT: usize = 3;
pub const ZMQ_THREAD_PRIORITY: usize = 3;
pub const ZMQ_THREAD_SCHED_POLICY: usize = 4;

// Default for new contexts
pub const ZMQ_IO_THREADS_DFLT: usize = 1;
pub const ZMQ_MAX_SOCKETS_DFLT: usize = 1023;
pub const ZMQ_THREAD_PRIORITY_DFLT: isize = -1;
pub const ZMQ_THREAD_SCHED_POLICY_DFLT: isize = -1;

//
// 0MQ message definition
//

#[repr(C)]
#[derive(Copy)]
pub struct Struct_zmq_msg_t {
    pub _m: [::libc::c_uchar; 64usize],
}

impl Clone for Struct_zmq_msg_t {
    fn clone(&self) -> Self { *self }
}

#[allow(non_camel_case_types)]
pub type zmq_msg_t = Struct_zmq_msg_t;

#[allow(non_camel_case_types)]
pub type zmq_free_fn =
    extern "C" fn(data: *mut ::libc::c_void, hint: *mut ::libc::c_void);

//
// 0MQ socket definition
//

// Socket types
pub const ZMQ_PAIR: usize = 0;
pub const ZMQ_PUB: usize = 1;
pub const ZMQ_SUB: usize = 2;
pub const ZMQ_REQ: usize = 3;
pub const ZMQ_REP: usize = 4;
pub const ZMQ_DEALER: usize = 5;
pub const ZMQ_ROUTER: usize = 6;
pub const ZMQ_PULL: usize = 7;
pub const ZMQ_PUSH: usize = 8;
pub const ZMQ_XPUB: usize = 9;
pub const ZMQ_XSUB: usize = 10;
pub const ZMQ_STREAM: usize = 11;
pub const ZMQ_SERVER: usize = 12;
pub const ZMQ_CLIENT: usize = 13;

// Deprecated aliases
pub const ZMQ_XREQ: usize = ZMQ_DEALER;
pub const ZMQ_XREP: usize = ZMQ_ROUTER;

// Socket options
pub const ZMQ_AFFINITY: usize = 4;
pub const ZMQ_IDENTITY: usize = 5;
pub const ZMQ_SUBSCRIBE: usize = 6;
pub const ZMQ_UNSUBSCRIBE: usize = 7;
pub const ZMQ_RATE: usize = 8;
pub const ZMQ_RECOVERY_IVL: usize = 9;
pub const ZMQ_SNDBUF: usize = 11;
pub const ZMQ_RCVBUF: usize = 12;
pub const ZMQ_RCVMORE: usize = 13;
pub const ZMQ_FD: usize = 14;
pub const ZMQ_EVENTS: usize = 15;
pub const ZMQ_TYPE: usize = 16;
pub const ZMQ_LINGER: usize = 17;
pub const ZMQ_RECONNECT_IVL: usize = 18;
pub const ZMQ_BACKLOG: usize = 19;
pub const ZMQ_RECONNECT_IVL_MAX: usize = 21;
pub const ZMQ_MAXMSGSIZE: usize = 22;
pub const ZMQ_SNDHWM: usize = 23;
pub const ZMQ_RCVHWM: usize = 24;
pub const ZMQ_MULTICAST_HOPS: usize = 25;
pub const ZMQ_RCVTIMEO: usize = 27;
pub const ZMQ_SNDTIMEO: usize = 28;
pub const ZMQ_LAST_ENDPOINT: usize = 32;
pub const ZMQ_ROUTER_MANDATORY: usize = 33;
pub const ZMQ_TCP_KEEPALIVE: usize = 34;
pub const ZMQ_TCP_KEEPALIVE_CNT: usize = 35;
pub const ZMQ_TCP_KEEPALIVE_IDLE: usize = 36;
pub const ZMQ_TCP_KEEPALIVE_INTVL: usize = 37;
pub const ZMQ_IMMEDIATE: usize = 39;
pub const ZMQ_XPUB_VERBOSE: usize = 40;
pub const ZMQ_ROUTER_RAW: usize = 41;
pub const ZMQ_IPV6: usize = 42;
pub const ZMQ_MECHANISM: usize = 43;
pub const ZMQ_PLAIN_SERVER: usize = 44;
pub const ZMQ_PLAIN_USERNAME: usize = 45;
pub const ZMQ_PLAIN_PASSWORD: usize = 46;
pub const ZMQ_CURVE_SERVER: usize = 47;
pub const ZMQ_CURVE_PUBLICKEY: usize = 48;
pub const ZMQ_CURVE_SECRETKEY: usize = 49;
pub const ZMQ_CURVE_SERVERKEY: usize = 50;
pub const ZMQ_PROBE_ROUTER: usize = 51;
pub const ZMQ_REQ_CORRELATE: usize = 52;
pub const ZMQ_REQ_RELAXED: usize = 53;
pub const ZMQ_CONFLATE: usize = 54;
pub const ZMQ_ZAP_DOMAIN: usize = 55;
pub const ZMQ_ROUTER_HANDOVER: usize = 56;
pub const ZMQ_TOS: usize = 57;
pub const ZMQ_CONNECT_RID: usize = 61;
pub const ZMQ_GSSAPI_SERVER: usize = 62;
pub const ZMQ_GSSAPI_PRINCIPAL: usize = 63;
pub const ZMQ_GSSAPI_SERVICE_PRINCIPAL: usize = 64;
pub const ZMQ_GSSAPI_PLAINTEXT: usize = 65;
pub const ZMQ_HANDSHAKE_IVL: usize = 66;
pub const ZMQ_SOCKS_PROXY: usize = 68;
pub const ZMQ_XPUB_NODROP: usize = 69;
pub const ZMQ_BLOCKY: usize = 70;
pub const ZMQ_XPUB_MANUAL: usize = 71;
pub const ZMQ_XPUB_WELCOME_MSG: usize = 72;
pub const ZMQ_STREAM_NOTIFY: usize = 73;
pub const ZMQ_INVERT_MATCHING: usize = 74;
pub const ZMQ_HEARTBEAT_IVL: usize = 75;
pub const ZMQ_HEARTBEAT_TTL: usize = 76;
pub const ZMQ_HEARTBEAT_TIMEOUT: usize = 77;

// Message options
pub const ZMQ_MORE: usize = 1;
pub const ZMQ_SRCFD: usize = 2;
pub const ZMQ_SHARED: usize = 3;

// Send/recv options
pub const ZMQ_DONTWAIT: usize = 1;
pub const ZMQ_SNDMORE: usize = 2;

// Security mechanisms
pub const ZMQ_NULL: usize = 0;
pub const ZMQ_PLAIN: usize = 1;
pub const ZMQ_CURVE: usize = 2;
pub const ZMQ_GSSAPI: usize = 3;

// Deprecated options and aliases
pub const ZMQ_TCP_ACCEPT_FILTER: usize = 38;
pub const ZMQ_IPC_FILTER_PID: usize = 58;
pub const ZMQ_IPC_FILTER_UID: usize = 59;
pub const ZMQ_IPC_FILTER_GID: usize = 60;
pub const ZMQ_IPV4ONLY: usize = 31;
pub const ZMQ_DELAY_ATTACH_ON_CONNECT: usize = ZMQ_IMMEDIATE;
pub const ZMQ_NOBLOCK: usize = ZMQ_DONTWAIT;
pub const ZMQ_FAIL_UNROUTABLE: usize = ZMQ_ROUTER_MANDATORY;
pub const ZMQ_ROUTER_BEHAVIOR: usize = ZMQ_ROUTER_MANDATORY;

//
// 0MQ socket events and monitoring
//

// Socket transport events (TCP and IPC only)
pub const ZMQ_EVENT_CONNECTED: usize = 1;
pub const ZMQ_EVENT_CONNECT_DELAYED: usize = 2;
pub const ZMQ_EVENT_CONNECT_RETRIED: usize = 4;
pub const ZMQ_EVENT_LISTENING: usize = 8;
pub const ZMQ_EVENT_BIND_FAILED: usize = 16;
pub const ZMQ_EVENT_ACCEPTED: usize = 32;
pub const ZMQ_EVENT_ACCEPT_FAILED: usize = 64;
pub const ZMQ_EVENT_CLOSED: usize = 128;
pub const ZMQ_EVENT_CLOSE_FAILED: usize = 256;
pub const ZMQ_EVENT_DISCONNECTED: usize = 512;
pub const ZMQ_EVENT_MONITOR_STOPPED: usize = 1024;
pub const ZMQ_EVENT_ALL: usize = 65536;

//
// I/O multiplexing
//
pub const ZMQ_POLLIN: usize = 1;
pub const ZMQ_POLLOUT: usize = 2;
pub const ZMQ_POLLERR: usize = 4;
pub const ZMQ_POLLPRI: usize = 8;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_zmq_pollitem_t {
    socket: *mut libc::c_void,
    fd: libc::c_int,
    events: libc::c_short,
    revents: libc::c_short,
}

impl Clone for Struct_zmq_pollitem_t {
    fn clone(&self) -> Self { *self }
}
impl Default for Struct_zmq_pollitem_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[allow(non_camel_case_types)]
pub type zmq_pollitem_t = Struct_zmq_pollitem_t;

pub const ZMQ_POLLITEMS_DFLT: usize = 16;

//
// Probe library capabilities
//
pub const ZMQ_HAS_CAPABILITIES: usize = 1;

// Deprecated aliases
pub const ZMQ_STREAMER: usize = 1;
pub const ZMQ_FORWARDER: usize = 2;
pub const ZMQ_QUEUE: usize = 3;


//
// These functions are not documented by man pages -- use at your own
// risk. If you need these to be part of the formal ZMQ API, then
// (a) write a man page, and (b) write a test case in tests.
//

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct iovec;

#[allow(non_camel_case_types)]
pub type zmq_thread_fn = extern "C" fn(arg: *mut libc::c_void);

#[link(name = "zmq")]
extern {
    // This function retrieves the errno as it is known to 0MQ library.
    // The goal of this function is to make the code 100% portable,
    // including where 0MQ compiled with certain
    // CRT library (on Windows) is linked to an application that uses
    // different CRT library.
    pub fn zmq_errno();

    // Resolves system errors and 0MQ errors to human-readable string
    pub fn zmq_strerror(errnum: &i32) -> *const libc::c_char;

    // Run-time API version detection
    pub fn zmq_version(major: &mut i32, minor: &mut i32, patch: &mut i32);

    // Context functions
    // New API
    pub fn zmq_ctx_new() -> *mut libc::c_void;
    pub fn zmq_ctx_term(context: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_ctx_shutdown(ctx_: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_ctx_set(context: *mut libc::c_void,
                       option: libc::c_int,
                       optval: libc::c_int) -> libc::c_int;
    pub fn zmq_ctx_get(context: *mut libc::c_void,
                       option: libc::c_int) -> libc::c_int;

    // Old (legacy) API
    pub fn zmq_init(io_threads: libc::c_int) -> *mut libc::c_void;
    pub fn zmq_term(context: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_ctx_destroy(context: *mut libc::c_void) -> libc::c_int;

    // Message functions
    pub fn zmq_msg_init(msg: *mut zmq_msg_t) -> libc::c_int;
    pub fn zmq_msg_init_size(msg: *mut zmq_msg_t, size: libc::size_t) -> libc::c_int;
    pub fn zmq_msg_init_data(msg: *mut zmq_msg_t,
                             data: *mut libc::c_void,
                             size: libc::size_t,
                             ffn: *mut zmq_free_fn,
                             hint: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_msg_send(msg: *mut zmq_msg_t, s: *mut libc::c_void,
                        flags: libc::c_int) -> libc::c_int;
    pub fn zmq_msg_recv(msg: *mut zmq_msg_t, s: *mut libc::c_void,
                        flags: libc::c_int) -> libc::c_int;
    pub fn zmq_msg_close(msg: *mut zmq_msg_t) -> libc::c_int;
    pub fn zmq_msg_move(dest: *mut zmq_msg_t,
                        src: *mut zmq_msg_t) -> libc::c_int;
    pub fn zmq_msg_copy(dest: *mut zmq_msg_t,
                        src: *mut zmq_msg_t) -> libc::c_int;
    pub fn zmq_msg_data(msg: *mut zmq_msg_t) -> *mut libc::c_void;
    pub fn zmq_msg_size(msg: *mut zmq_msg_t) -> libc::size_t;
    pub fn zmq_msg_more(msg: *mut zmq_msg_t) -> libc::c_int;
    pub fn zmq_msg_get(msg: *mut zmq_msg_t,
                       property: libc::c_int) -> libc::c_int;
    pub fn zmq_msg_set(msg: *mut zmq_msg_t,
                       property: libc::c_int,
                       optval: libc::c_int) -> libc::c_int;
    pub fn zmq_msg_gets(msg: *mut zmq_msg_t,
                        property: *const libc::c_char) -> *const libc::c_char;
    pub fn zmq_msg_set_routing_id(msg: *mut zmq_msg_t,
                                  routing_id: libc::uint32_t) -> libc::c_int;
    pub fn zmq_msg_get_routing_id(msg: *mut zmq_msg_t) -> libc::uint32_t;

    // Socket functions
    pub fn zmq_socket(s: *mut libc::c_void, t: libc::c_int) -> *mut libc::c_void;
    pub fn zmq_close(s: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_setsockopt(s: *mut libc::c_void,
                          option: libc::c_int,
                          optval: *const libc::c_void,
                          optvallen: libc::size_t) -> libc::c_int;
    pub fn zmq_getsockopt(s: *mut libc::c_void,
                          option: libc::c_int,
                          optval: *mut libc::c_void,
                          optvallen: *mut libc::size_t) -> libc::c_int;
    pub fn zmq_bind(s: *mut libc::c_void,
                    addr: *const libc::c_char) -> libc::c_int;
    pub fn zmq_connect(s: *mut libc::c_void,
                       addr: *const libc::c_char) -> libc::c_int;
    pub fn zmq_unbind(s: *mut libc::c_void,
                      addr: *const libc::c_char) -> libc::c_int;
    pub fn zmq_disconnect(s: *mut libc::c_void,
                          addr: *const libc::c_char) -> libc::c_int;
    pub fn zmq_send(s: *mut libc::c_void,
                    buf: *const libc::c_void,
                    len: libc::size_t,
                    flags: libc::c_int) -> libc::c_int;
    pub fn zmq_send_const(s: *mut libc::c_void,
                          buf: *const libc::c_void,
                          len: libc::size_t,
                          flags: libc::c_int) -> libc::c_int;
    pub fn zmq_recv(s: *mut libc::c_void,
                    buf: *mut libc::c_void,
                    len: libc::size_t,
                    flags: libc::c_int) -> libc::c_int;
    pub fn zmq_socket_monitor(s: *mut libc::c_void,
                              addr: *const libc::c_char,
                              events: libc::c_int) -> libc::c_int;

    // I/O multiplexing function
    pub fn zmq_poll(items: *mut zmq_pollitem_t,
                    nitems: libc::c_int,
                    timeout: libc::c_long) -> libc::c_int;

    // Message proxying functions
    pub fn zmq_proxy(frontend: *mut libc::c_void,
                     backend: *mut libc::c_void,
                     capture: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_proxy_steerable(frontend: *mut libc::c_void,
                               backend: *mut libc::c_void,
                               capture: *mut libc::c_void,
                               control: *mut libc::c_void) -> libc::c_int;

    // Probe library capabilities functions
    pub fn zmq_has(capabitility: *const libc::c_char) -> libc::c_int;
    // Deprecated methods
    pub fn zmq_device(t: libc::c_int,
                      frontend: *mut libc::c_void,
                      backend: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_sendmsg(s: *mut libc::c_void,
                       msg: *mut zmq_msg_t,
                       flags: libc::c_int) -> libc::c_int;
    pub fn zmq_recvmsg(s: *mut libc::c_void,
                       msg: *mut zmq_msg_t,
                       flags: libc::c_int) -> libc::c_int;

    // Encryption functions
    // Encode data with Z85 encoding. Returns encoded data
    pub fn zmq_z85_encode(dest: *mut libc::c_char,
                          data: *const libc::uint8_t,
                          size: libc::size_t) -> *mut libc::c_char;

    // Decode data with Z85 encoding. Returns decoded data
    pub fn zmq_z85_decode(dest: *mut libc::uint8_t,
                          string: *const libc::c_char) -> libc::uint8_t;

    // Generate z85-encoded public and private keypair with libsodium.
    // Returns 0 on success.
    pub fn zmq_curve_keypair(z85_public_key: *mut libc::c_char,
                             z85_secret_key: *mut libc::c_char) -> libc::c_int;

    // Atomic utility functions
    pub fn zmq_atomic_counter_new() -> *mut libc::c_void;
    pub fn zmq_atomic_counter_set(counter: *mut libc::c_void, value: libc::c_int);
    pub fn zmq_atomic_counter_inc(counter: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_atomic_counter_dec(counter: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_atomic_counter_value(counter: *mut libc::c_void) -> libc::c_int;
    pub fn zmq_atomic_counter_destroy(counter: *mut *mut libc::c_void);

    // Helper functions are used by perf tests so that they don't have to care
    // about minutiae of time-related functions on different OS platforms.
    // Starts the stopwatch. Returns the handle to the watch
    pub fn zmq_stopwatch_start() -> *mut libc::c_void;

    // Stops the stopwatch. Returns the number of microseconds elapsed since
    // the stopwatch was started
    pub fn zmq_stopwatch_stop(watch_: *mut libc::c_void) -> libc::c_ulong;

    // Sleeps for specified number of seconds
    pub fn zmq_sleep(seconds_: libc::c_int);

    // Start a thread. Returns a handle to the thread
    pub fn zmq_threadstart(func: *mut zmq_thread_fn, arg: *mut libc::c_void);

    // Wait for thread to complete then free up resources
    pub fn zmq_threadclose(thread: *mut libc::c_void);

    // Not documented function
    pub fn zmq_sendiov(s: *mut libc::c_void,
                       iov: *mut iovec,
                       count: libc::size_t,
                       flags: libc::c_int) -> libc::c_int;
    pub fn zmq_recviov(s: *mut libc::c_void,
                       iov: *mut iovec,
                       count: *mut libc::size_t,
                       flags: libc::c_int) -> libc::c_int;
}
