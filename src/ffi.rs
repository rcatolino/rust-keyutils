extern crate libc;

#[allow(non_camel_case_types)]
pub type key_serial_t = libc::int32_t;

pub const KEY_SPEC_THREAD_KEYRING:          key_serial_t = -1;
pub const KEY_SPEC_PROCESS_KEYRING:         key_serial_t = -2;
pub const KEY_SPEC_SESSION_KEYRING:         key_serial_t = -3;
pub const KEY_SPEC_USER_KEYRING:            key_serial_t = -4;
pub const KEY_SPEC_USER_SESSION_KEYRING:    key_serial_t = -5;
pub const KEY_SPEC_GROUP_KEYRING:           key_serial_t = -6;
pub const KEY_SPEC_REQKEY_AUTH_KEY:         key_serial_t = -7;

pub const KEY_REQKEY_DEFL_NO_CHANGE:            key_serial_t = -1;
pub const KEY_REQKEY_DEFL_DEFAULT:              key_serial_t = 0;
pub const KEY_REQKEY_DEFL_THREAD_KEYRING:       key_serial_t = 1;
pub const KEY_REQKEY_DEFL_PROCESS_KEYRING:      key_serial_t = 2;
pub const KEY_REQKEY_DEFL_SESSION_KEYRING:      key_serial_t = 3;
pub const KEY_REQKEY_DEFL_USER_KEYRING:         key_serial_t = 4;
pub const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: key_serial_t = 5;
pub const KEY_REQKEY_DEFL_GROUP_KEYRING:        key_serial_t = 6;

#[allow(non_camel_case_types)]
pub type key_perm_t = libc::uint32_t;

pub const KEY_POS_VIEW:    key_perm_t = 0x01000000;     /* possessor can view a key's attributes */
pub const KEY_POS_READ:    key_perm_t = 0x02000000;     /* possessor can read key payload / view keyring */
pub const KEY_POS_WRITE:   key_perm_t = 0x04000000;     /* possessor can update key payload / add link to keyring */
pub const KEY_POS_SEARCH:  key_perm_t = 0x08000000;     /* possessor can find a key in search / search a keyring */
pub const KEY_POS_LINK:    key_perm_t = 0x10000000;     /* possessor can create a link to a key/keyring */
pub const KEY_POS_SETATTR: key_perm_t = 0x20000000;     /* possessor can set key attributes */
pub const KEY_POS_ALL:     key_perm_t = 0x3f000000;

pub const KEY_USR_VIEW:    key_perm_t = 0x00010000;     /* user permissions... */
pub const KEY_USR_READ:    key_perm_t = 0x00020000;
pub const KEY_USR_WRITE:   key_perm_t = 0x00040000;
pub const KEY_USR_SEARCH:  key_perm_t = 0x00080000;
pub const KEY_USR_LINK:    key_perm_t = 0x00100000;
pub const KEY_USR_SETATTR: key_perm_t = 0x00200000;
pub const KEY_USR_ALL:     key_perm_t = 0x003f0000;

pub const KEY_GRP_VIEW:    key_perm_t = 0x00000100;     /* group permissions... */
pub const KEY_GRP_READ:    key_perm_t = 0x00000200;
pub const KEY_GRP_WRITE:   key_perm_t = 0x00000400;
pub const KEY_GRP_SEARCH:  key_perm_t = 0x00000800;
pub const KEY_GRP_LINK:    key_perm_t = 0x00001000;
pub const KEY_GRP_SETATTR: key_perm_t = 0x00002000;
pub const KEY_GRP_ALL:     key_perm_t = 0x00003f00;

pub const KEY_OTH_VIEW:    key_perm_t = 0x00000001;     /* third party permissions... */
pub const KEY_OTH_READ:    key_perm_t = 0x00000002;
pub const KEY_OTH_WRITE:   key_perm_t = 0x00000004;
pub const KEY_OTH_SEARCH:  key_perm_t = 0x00000008;
pub const KEY_OTH_LINK:    key_perm_t = 0x00000010;
pub const KEY_OTH_SETATTR: key_perm_t = 0x00000020;
pub const KEY_OTH_ALL:     key_perm_t = 0x0000003f;

// No actual type in the API, but create one for simplicity.
#[allow(non_camel_case_types)]
pub type _keyctl_support_t = libc::uint32_t;

pub const KEYCTL_SUPPORTS_ENCRYPT: _keyctl_support_t = 0x01;
pub const KEYCTL_SUPPORTS_DECRYPT: _keyctl_support_t = 0x02;
pub const KEYCTL_SUPPORTS_SIGN:    _keyctl_support_t = 0x04;
pub const KEYCTL_SUPPORTS_VERIFY:  _keyctl_support_t = 0x08;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct keyctl_pkey_query {
    pub supported_ops:  libc::uint32_t,
    pub key_size:       libc::uint32_t,
    pub max_data_size:  libc::uint16_t,
    pub max_sig_size:   libc::uint16_t,
    pub max_enc_size:   libc::uint16_t,
    pub max_dec_size:   libc::uint16_t,
    __spare:            [libc::uint32_t; 10],
}

impl keyctl_pkey_query {
    pub fn new() -> Self {
        keyctl_pkey_query {
            supported_ops: 0,
            key_size: 0,
            max_data_size: 0,
            max_sig_size: 0,
            max_enc_size: 0,
            max_dec_size: 0,
            __spare: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

#[link(name = "keyutils")]
extern {
    pub fn add_key(
        type_:          *const libc::c_char,
        description:    *const libc::c_char,
        payload:        *const libc::c_void,
        plen:           libc::size_t,
        keyring:        key_serial_t)
        -> key_serial_t;
    pub fn request_key(
        type_:          *const libc::c_char,
        description:    *const libc::c_char,
        callout_info:   *const libc::c_char,
        keyring:        key_serial_t)
        -> key_serial_t;

    pub fn keyctl_get_keyring_ID(
        id:     key_serial_t,
        create: libc::c_int)
        -> key_serial_t;
    pub fn keyctl_join_session_keyring(
        name:   *const libc::c_char)
        -> key_serial_t;
    pub fn keyctl_update(
        id:         key_serial_t,
        payload:    *const libc::c_void,
        plen:       libc::size_t)
        -> libc::c_long;
    pub fn keyctl_revoke(
        id: key_serial_t)
        -> libc::c_long;
    pub fn keyctl_chown(
        id:     key_serial_t,
        uid:    libc::uid_t,
        gid:    libc::gid_t)
        -> libc::c_long;
    pub fn keyctl_setperm(
        id:     key_serial_t,
        perm:   key_perm_t)
        -> libc::c_long;
    pub fn keyctl_describe(
        id:     key_serial_t,
        buffer: *mut libc::c_char,
        buflen: libc::size_t)
        -> libc::c_long;
    pub fn keyctl_clear(
        ringid: key_serial_t)
        -> libc::c_long;
    pub fn keyctl_link(
        id:     key_serial_t,
        ringid: key_serial_t)
        -> libc::c_long;
    pub fn keyctl_unlink(
        id:     key_serial_t,
        ringid: key_serial_t)
        -> libc::c_long;
    pub fn keyctl_search(
        ringid:         key_serial_t,
        type_:          *const libc::c_char,
        description:    *const libc::c_char,
        destringid:     key_serial_t)
        -> libc::c_long;
    pub fn keyctl_read(
        id:     key_serial_t,
        buffer: *mut libc::c_char,
        buflen: libc::size_t)
        -> libc::c_long;
    pub fn keyctl_instantiate(
        id:         key_serial_t,
        payload:    *const libc::c_void,
        plen:       libc::size_t,
        ringid:     key_serial_t)
        -> libc::c_long;
    pub fn keyctl_negate(
        id:         key_serial_t,
        timeout:    libc::c_uint,
        ringid:     key_serial_t)
        -> libc::c_long;
    pub fn keyctl_set_reqkey_keyring(
        reqkey_defl:    libc::c_int)
        -> libc::c_long;
    pub fn keyctl_set_timeout(
        key:        key_serial_t,
        timeout:    libc::c_uint)
        -> libc::c_long;
    pub fn keyctl_assume_authority(
        key:    key_serial_t)
        -> libc::c_long;
    pub fn keyctl_get_security(
        key:    key_serial_t,
        buffer: *mut libc::c_char,
        buflen: libc::size_t)
        -> libc::c_long;
    //pub fn keyctl_session_to_parent()
    //    -> libc::c_long;
    pub fn keyctl_reject(
        id:         key_serial_t,
        timeout:    libc::c_uint,
        error:      libc::c_uint,
        ringid:     key_serial_t)
        -> libc::c_long;
    pub fn keyctl_invalidate(
        id: key_serial_t)
        -> libc::c_long;
    pub fn keyctl_get_persistent(
        uid:    libc::uid_t,
        id:     key_serial_t)
        -> libc::c_long;
    pub fn keyctl_dh_compute(
        private:    key_serial_t,
        prime:      key_serial_t,
        base:       key_serial_t,
        buffer:     *mut libc::c_char,
        buflen:     libc::size_t)
        -> libc::c_long;
    pub fn keyctl_pkey_query(
        key:        key_serial_t,
        password:   key_serial_t,
        info:       *mut keyctl_pkey_query)
        -> libc::c_long;
    pub fn keyctl_pkey_encrypt(
        key:        key_serial_t,
        password:   key_serial_t,
        info:       *const libc::c_char,
        data:       *const libc::c_void,
        data_len:   libc::size_t,
        enc:        *mut libc::c_void,
        enc_len:    libc::size_t)
        -> libc::c_long;
    pub fn keyctl_pkey_decrypt(
        key:        key_serial_t,
        password:   key_serial_t,
        info:       *const libc::c_char,
        enc:        *const libc::c_void,
        enc_len:    libc::size_t,
        data:       *mut libc::c_void,
        data_len:   libc::size_t)
        -> libc::c_long;
    pub fn keyctl_pkey_sign(
        key:        key_serial_t,
        password:   key_serial_t,
        info:       *const libc::c_char,
        data:       *const libc::c_void,
        data_len:   libc::size_t,
        sig:        *mut libc::c_void,
        sig_len:    libc::size_t)
        -> libc::c_long;
    pub fn keyctl_pkey_verify(
        key:        key_serial_t,
        password:   key_serial_t,
        info:       *const libc::c_char,
        data:       *const libc::c_void,
        data_len:   libc::size_t,
        sig:        *const libc::c_void,
        sig_len:    libc::size_t)
        -> libc::c_long;
}
