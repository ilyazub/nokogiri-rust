use libc::c_char;
use scraper::{ElementRef, Html, Selector};
use std::ffi::{CStr, CString};

pub struct NokogiriRust {
    document: Html,
}

pub struct Node<'a> {
    elementRef: ElementRef<'a>,
}

impl NokogiriRust {
    fn parse(html: &str) -> Self {
        NokogiriRust {
            document: Html::parse_document(html),
        }
    }

    fn at_css(&self, selector: &str) -> Node {
        Node::new(self
            .document
            .select(&Selector::parse(selector).unwrap())
            .next()
            .unwrap())
    }
}


impl<'a> Node<'a> {
    fn new(node: NodeRef<'a, Node>) -> Self {
        ElementRef { node }
    }
}

#[no_mangle]
pub extern "C" fn nokogiri_rust_parse(html: *const c_char) -> *mut NokogiriRust {
    let html = unsafe {
        assert!(!html.is_null());
        CStr::from_ptr(html).to_str().unwrap()
    };

    Box::into_raw(Box::new(NokogiriRust::parse(&html)))
}

#[no_mangle]
pub extern "C" fn nokogiri_rust_free(ptr: *mut NokogiriRust) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn nokogiri_rust_at_css(
    ptr: *const NokogiriRust,
    selector: *const c_char,
) -> *mut NokogiriRust {
    let nokogiri_rust = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let selector = unsafe {
        assert!(!selector.is_null());
        CStr::from_ptr(selector)
    };

    let selector_str = selector.to_str().unwrap();
    let node = nokogiri_rust.at_css(selector_str);

    Box::into_raw(Box::new(NokogiriRust::parse(&html)))
    
    // CString::new(result).unwrap().into_raw()
}
