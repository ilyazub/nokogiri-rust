use scraper::{Html, Selector};

#[macro_use]
extern crate rutie;

#[macro_use]
extern crate lazy_static;

use rutie::{AnyObject, Class, Module, Object, RString, VerifiedObject, VM};

wrappable_struct!(Html, HtmlWrapper, HTML_WRAPPER);

class!(NokogiriRust);
class!(Document);

methods!(
    NokogiriRust,
    _itself,

    fn html(html_string: RString) -> AnyObject {
        Document::new_document(html_string)
    }
);

methods!(
    Document,
    _itself,

    fn new_document(html_string: RString) -> AnyObject {
        let html = Html::parse_document(html_string.unwrap().to_str());

        Class::from_existing("Document").wrap_data(html, &*HTML_WRAPPER)
    }

    fn at_css(selector: RString) -> RString {
        let result = _itself
            .get_data(&*HTML_WRAPPER)
            .select(&Selector::parse(selector.unwrap().to_str()).unwrap())
            .next()
            .unwrap()
            .inner_html();

        RString::new_utf8(&result)
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_nokogiri_rust() {
    Module::from_existing("NokogiriRust").define(|klass| {
        klass.def_self("HTML", html);

        let data_class = Class::from_existing("Object");
        klass.define_nested_class("Document", Some(&data_class)).define(|nested_klass| {
            nested_klass.def_self("new", new_document);
            nested_klass.def("at_css", at_css);
        });
    });
}
