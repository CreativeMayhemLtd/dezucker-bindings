use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use std::ptr;
use crate::types::{FormattedPost, IdField, PostFragment};

/// C-compatible representation of IdField
#[repr(C)]
pub enum CIdFieldTag {
    STRING,
    NUMBER,
}

#[repr(C)]
pub struct CIdField {
    pub tag: CIdFieldTag,
    pub string_val: *mut c_char,
    pub number_val: i64,
}

/// C-compatible representation of PostFragment
#[repr(C)]
pub struct CPostFragment {
    pub text: *mut c_char,
    pub timestamp: i64,
    pub media_uri: *mut c_char,
    pub web_uri: *mut c_char,
    pub is_photo: bool,
    pub is_unknown: bool,
    pub is_meaningful: bool,
}

/// C-compatible representation of FormattedPost
#[repr(C)]
pub struct CFormattedPost {
    pub tid: *mut c_char,
    pub id: *mut CIdField,
    pub text: *mut c_char,
    pub timestamp: *mut CIdField,
    pub attachments_count: i32,
    pub has_attachments_count: bool,
    pub meaningful_entries_count: i32,
    pub fragments: *mut CPostFragment,
    pub fragments_len: usize,
    pub media: *mut CPostFragment,
    pub media_len: usize,
}

#[no_mangle]
pub extern "C" fn dezucker_free_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dezucker_free_post(post: *mut CFormattedPost) {
    if post.is_null() { return; }
    let p = Box::from_raw(post);
    
    dezucker_free_string(p.tid);
    dezucker_free_string(p.text);
    
    if !p.id.is_null() {
        let id = Box::from_raw(p.id);
        dezucker_free_string(id.string_val);
    }
    
    if !p.timestamp.is_null() {
        let ts = Box::from_raw(p.timestamp);
        dezucker_free_string(ts.string_val);
    }
    
    if !p.fragments.is_null() {
        let fragments = Vec::from_raw_parts(p.fragments, p.fragments_len, p.fragments_len);
        for f in fragments {
            dezucker_free_string(f.text);
            dezucker_free_string(f.media_uri);
            dezucker_free_string(f.web_uri);
        }
    }

    if !p.media.is_null() {
        let media = Vec::from_raw_parts(p.media, p.media_len, p.media_len);
        for m in media {
            dezucker_free_string(m.text);
            dezucker_free_string(m.media_uri);
            dezucker_free_string(m.web_uri);
        }
    }
}
