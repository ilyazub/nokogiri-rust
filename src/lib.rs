use libc::c_char;
use scraper::{ElementRef, Html, Selector};
use std::ffi::{CStr, CString};

pub struct NokogiriRust {
    document: Html,
}

impl NokogiriRust {
    fn parse(html: &str) -> Self {
        NokogiriRust {
            document: Html::parse_document(html),
        }
    }

    fn at_css(&self, selector: &str) -> ElementRef {
        self.document
            .select(&Selector::parse(selector).unwrap())
            .next()
            .unwrap()
    }
}

#[no_mangle]
pub extern "C" fn nokogiri_rust_html_parse(html: *const c_char) -> *mut NokogiriRust {
    let html = unsafe {
        assert!(!html.is_null());
        CStr::from_ptr(html).to_str().unwrap()
    };

    Box::into_raw(Box::new(NokogiriRust::parse(&html)))
}

#[no_mangle]
pub extern "C" fn nokogiri_rust_html_free(ptr: *mut NokogiriRust) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn nokogiri_rust_element_ref_free(ptr: *mut ElementRef<'static>) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn nokogiri_rust_html_at_css(
    ptr: *const NokogiriRust,
    selector: *const c_char,
) -> *mut ElementRef<'static> {
    let nokogiri_rust = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let selector = unsafe {
        assert!(!selector.is_null());
        CStr::from_ptr(selector)
    };

    let selector_str = selector.to_str().unwrap();
    let element_ref = nokogiri_rust.at_css(selector_str);

    Box::into_raw(Box::new(element_ref))
}

#[no_mangle]
pub extern "C" fn nokogiri_rust_element_ref_text(
    ptr: *const ElementRef,
) -> *const c_char {
    let element_ref = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let text = element_ref.text().next().unwrap();

    CString::new(text).unwrap().into_raw()
}
