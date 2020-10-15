// Copied from generated bindings so bindgen is not needed just to compile
// This must be updated if any of the PacketCrypt headers change (!!)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_BlockHeader_t {
    pub version: u32,
    pub hashPrevBlock: [u32; 8usize],
    pub hashMerkleRoot: [u32; 8usize],
    pub timeSeconds: u32,
    pub workBits: u32,
    pub nonce: u32,
}
#[test]
fn bindgen_test_layout_PacketCrypt_BlockHeader_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_BlockHeader_t>(),
        80usize,
        concat!("Size of: ", stringify!(PacketCrypt_BlockHeader_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_BlockHeader_t>(),
        4usize,
        concat!("Alignment of ", stringify!(PacketCrypt_BlockHeader_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).version as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).hashPrevBlock as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(hashPrevBlock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).hashMerkleRoot as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(hashMerkleRoot)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).timeSeconds as *const _ as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(timeSeconds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).workBits as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(workBits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_BlockHeader_t>())).nonce as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_BlockHeader_t),
            "::",
            stringify!(nonce)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_AnnounceHdr_t {
    pub version: u8,
    pub softNonce: [u8; 3usize],
    pub hardNonce: u32,
    pub workBits: u32,
    pub parentBlockHeight: u32,
    pub contentType: u32,
    pub contentLength: u32,
    pub contentHash: [u8; 32usize],
    pub signingKey: [u8; 32usize],
}
#[test]
fn bindgen_test_layout_PacketCrypt_AnnounceHdr_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_AnnounceHdr_t>(),
        88usize,
        concat!("Size of: ", stringify!(PacketCrypt_AnnounceHdr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_AnnounceHdr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(PacketCrypt_AnnounceHdr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).version as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).softNonce as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(softNonce)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).hardNonce as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(hardNonce)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).workBits as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(workBits)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).parentBlockHeight as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(parentBlockHeight)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).contentType as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(contentType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).contentLength as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(contentLength)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).contentHash as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(contentHash)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_AnnounceHdr_t>())).signingKey as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_AnnounceHdr_t),
            "::",
            stringify!(signingKey)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PacketCrypt_Announce_t {
    pub hdr: PacketCrypt_AnnounceHdr_t,
    pub proof: [u64; 117usize],
}
#[test]
fn bindgen_test_layout_PacketCrypt_Announce_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_Announce_t>(),
        1024usize,
        concat!("Size of: ", stringify!(PacketCrypt_Announce_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_Announce_t>(),
        8usize,
        concat!("Alignment of ", stringify!(PacketCrypt_Announce_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Announce_t>())).hdr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Announce_t),
            "::",
            stringify!(hdr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Announce_t>())).proof as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Announce_t),
            "::",
            stringify!(proof)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PacketCrypt_HeaderAndProof_t {
    pub blockHeader: PacketCrypt_BlockHeader_t,
    pub _pad: u32,
    pub nonce2: u32,
    pub announcements: [PacketCrypt_Announce_t; 4usize],
    pub proof: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_PacketCrypt_HeaderAndProof_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_HeaderAndProof_t>(),
        4192usize,
        concat!("Size of: ", stringify!(PacketCrypt_HeaderAndProof_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_HeaderAndProof_t>(),
        8usize,
        concat!("Alignment of ", stringify!(PacketCrypt_HeaderAndProof_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>())).blockHeader as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(blockHeader)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>()))._pad as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(_pad)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>())).nonce2 as *const _ as usize
        },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(nonce2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>())).announcements as *const _
                as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(announcements)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_HeaderAndProof_t>())).proof as *const _ as usize
        },
        4184usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_HeaderAndProof_t),
            "::",
            stringify!(proof)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_Coinbase_t {
    pub magic: u32,
    pub annLeastWorkTarget: u32,
    pub merkleRoot: [u8; 32usize],
    pub numAnns: u64,
}
#[test]
fn bindgen_test_layout_PacketCrypt_Coinbase_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_Coinbase_t>(),
        48usize,
        concat!("Size of: ", stringify!(PacketCrypt_Coinbase_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_Coinbase_t>(),
        8usize,
        concat!("Alignment of ", stringify!(PacketCrypt_Coinbase_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Coinbase_t>())).magic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Coinbase_t),
            "::",
            stringify!(magic)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_Coinbase_t>())).annLeastWorkTarget as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Coinbase_t),
            "::",
            stringify!(annLeastWorkTarget)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PacketCrypt_Coinbase_t>())).merkleRoot as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Coinbase_t),
            "::",
            stringify!(merkleRoot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Coinbase_t>())).numAnns as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Coinbase_t),
            "::",
            stringify!(numAnns)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_Find_t {
    pub ptr: u64,
    pub size: u64,
}
#[test]
fn bindgen_test_layout_PacketCrypt_Find_t() {
    assert_eq!(
        ::std::mem::size_of::<PacketCrypt_Find_t>(),
        16usize,
        concat!("Size of: ", stringify!(PacketCrypt_Find_t))
    );
    assert_eq!(
        ::std::mem::align_of::<PacketCrypt_Find_t>(),
        8usize,
        concat!("Alignment of ", stringify!(PacketCrypt_Find_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Find_t>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Find_t),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PacketCrypt_Find_t>())).size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PacketCrypt_Find_t),
            "::",
            stringify!(size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PacketCrypt_ValidateCtx_s {
    _unused: [u8; 0],
}
pub type PacketCrypt_ValidateCtx_t = PacketCrypt_ValidateCtx_s;
extern "C" {
    pub fn ValidateCtx_create() -> *mut PacketCrypt_ValidateCtx_t;
}
extern "C" {
    pub fn ValidateCtx_destroy(ctx: *mut PacketCrypt_ValidateCtx_t);
}
pub const Validate_checkAnn_OK: ::std::os::raw::c_uint = 0;
pub const Validate_checkAnn_INVAL: ::std::os::raw::c_uint = 1;
pub const Validate_checkAnn_INVAL_ITEM4: ::std::os::raw::c_uint = 2;
pub const Validate_checkAnn_INSUF_POW: ::std::os::raw::c_uint = 3;
pub const Validate_checkAnn_SOFT_NONCE_HIGH: ::std::os::raw::c_uint = 4;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
extern "C" {
    pub fn Validate_checkAnn_outToString(
        code: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Validate_checkAnn(
        annHashOut: *mut u8,
        pcAnn: *const PacketCrypt_Announce_t,
        parentBlockHash: *const u8,
        vctx: *mut PacketCrypt_ValidateCtx_t,
    ) -> ::std::os::raw::c_int;
}
pub const Validate_checkBlock_OK: ::std::os::raw::c_uint = 0;
pub const Validate_checkBlock_SHARE_OK: ::std::os::raw::c_uint = 256;
pub const Validate_checkBlock_ANN_INVALID_: ::std::os::raw::c_uint = 512;
pub const Validate_checkBlock_ANN_INSUF_POW_: ::std::os::raw::c_uint = 768;
pub const Validate_checkBlock_ANN_SIG_INVALID_: ::std::os::raw::c_uint = 1024;
pub const Validate_checkBlock_ANN_CONTENT_INVALID_: ::std::os::raw::c_uint = 1280;
pub const Validate_checkBlock_PCP_INVAL: ::std::os::raw::c_uint = 1536;
pub const Validate_checkBlock_PCP_MISMATCH: ::std::os::raw::c_uint = 1792;
pub const Validate_checkBlock_INSUF_POW: ::std::os::raw::c_uint = 2048;
pub const Validate_checkBlock_BAD_COINBASE: ::std::os::raw::c_uint = 2304;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
extern "C" {
    pub fn Validate_checkBlock_outToString(
        code: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Validate_checkBlock(
        hap: *const PacketCrypt_HeaderAndProof_t,
        hapLen: u32,
        blockHeight: u32,
        shareTarget: u32,
        coinbaseCommitment: *const PacketCrypt_Coinbase_t,
        blockHashes: *const u8,
        workHashOut: *mut u8,
        vctx: *mut PacketCrypt_ValidateCtx_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AnnMiner_s {
    _unused: [u8; 0],
}
pub type AnnMiner_t = AnnMiner_s;
pub type AnnMiner_Callback = ::std::option::Option<
    unsafe extern "C" fn(callback_ctx: *mut ::std::os::raw::c_void, ann_buf: *mut u8),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AnnMiner_Request_s {
    pub workTarget: u32,
    pub parentBlockHeight: u32,
    pub parentBlockHash: [u8; 32usize],
    pub signingKey: [u8; 32usize],
    pub maxAnnsPerSecond: u32,
    pub contentType: u32,
    pub contentLen: u32,
}
#[test]
fn bindgen_test_layout_AnnMiner_Request_s() {
    assert_eq!(
        ::std::mem::size_of::<AnnMiner_Request_s>(),
        84usize,
        concat!("Size of: ", stringify!(AnnMiner_Request_s))
    );
    assert_eq!(
        ::std::mem::align_of::<AnnMiner_Request_s>(),
        4usize,
        concat!("Alignment of ", stringify!(AnnMiner_Request_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AnnMiner_Request_s>())).workTarget as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(workTarget)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AnnMiner_Request_s>())).parentBlockHeight as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(parentBlockHeight)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AnnMiner_Request_s>())).parentBlockHash as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(parentBlockHash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AnnMiner_Request_s>())).signingKey as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(signingKey)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AnnMiner_Request_s>())).maxAnnsPerSecond as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(maxAnnsPerSecond)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AnnMiner_Request_s>())).contentType as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(contentType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AnnMiner_Request_s>())).contentLen as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(AnnMiner_Request_s),
            "::",
            stringify!(contentLen)
        )
    );
}
pub type AnnMiner_Request_t = AnnMiner_Request_s;
extern "C" {
    pub fn AnnMiner_create(
        minerId: u32,
        threads: ::std::os::raw::c_int,
        callback_ctx: *mut ::std::os::raw::c_void,
        ann_found: AnnMiner_Callback,
    ) -> *mut AnnMiner_t;
}
extern "C" {
    pub fn AnnMiner_start(
        ctx: *mut AnnMiner_t,
        req: *mut AnnMiner_Request_t,
        version: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn AnnMiner_stop(miner: *mut AnnMiner_t);
}
extern "C" {
    pub fn AnnMiner_free(miner: *mut AnnMiner_t);
}
extern "C" {
    pub fn AnnMiner_getAnnsPerSecond(ctx: *const AnnMiner_t) -> f64;
}
