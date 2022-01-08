use scraper::{ElementRef, Html, Selector};

use rutie::{class, methods, module, Class, Module, Object, RString, VM};

// #[macro_use] extern crate lazy_static;
#[macro_use]
extern crate rutie;

// macro_rules! wrappable_struct {
//     (@mark_function_pointer) => {
//         None as Option<extern "C" fn(*mut rutie::types::c_void)>
//     };
//     // Leading comma is the comma between `$static_name: ident` and `mark` in the main macro rule.
//     // Optional comma `$(,)*` is not allowed in the main rule, because it is
//     // followed by `$($tail: tt)*`
//     (@mark_function_pointer , mark($object: ident) $body: block) => {
//         Some(Self::mark as extern "C" fn(*mut rutie::types::c_void))
//     };
//     (@mark_function_definition $struct_name: ty) => {};
//     (@mark_function_definition $struct_name: ty, mark($object: ident) $body: expr) => {
//         pub extern "C" fn mark(data: *mut rutie::types::c_void) {
//             let mut data = unsafe { (data as *mut $struct_name).as_mut() };

//             if let Some(ref mut $object) = data {
//                 $body
//             }
//         }
//     };
//     ($struct_name: ty, $lt: lifetime, $wrapper: ident, $static_name: ident $($tail: tt)*) => {
//         pub struct $wrapper<$lt, T> {
//             data_type: rutie::types::DataType,
//             _marker: ::std::marker::PhantomData<T>,
//         }

//         ::lazy_static::lazy_static! {
//             pub static ref $static_name: $wrapper<$lt, $struct_name> = $wrapper::new();
//         }

//         impl<$lt, T> $wrapper<$lt, T> {
//             fn new() -> $wrapper<$lt, T> {
//                 let name = concat!("Rutie/", stringify!($struct_name));
//                 let name = rutie::util::str_to_cstring(name);
//                 let reserved_bytes: [*mut rutie::types::c_void; 2] = [::std::ptr::null_mut(); 2];

//                 let dmark = wrappable_struct!(@mark_function_pointer $($tail)*);

//                 let data_type = rutie::types::DataType {
//                     wrap_struct_name: name.into_raw(),
//                     parent: ::std::ptr::null(),
//                     data: ::std::ptr::null_mut(),
//                     flags: rutie::types::Value::from(0),

//                     function: rutie::types::DataTypeFunction {
//                         dmark: dmark,
//                         dfree: Some(rutie::typed_data::free::<T>),
//                         dsize: None,
//                         reserved: reserved_bytes,
//                     },
//                 };

//                 $wrapper {
//                     data_type: data_type,
//                     _marker: ::std::marker::PhantomData,
//                 }
//             }

//             wrappable_struct!(@mark_function_definition $struct_name $($tail)*);
//         }

//         unsafe impl<$lt, T> Sync for $wrapper<$lt, T> {}

//         // Set constraint to be able to wrap and get data only for type `T`
//         impl<$lt, T> rutie::typed_data::DataTypeWrapper<T> for $wrapper<$lt, T> {
//             fn data_type(&self) -> &rutie::types::DataType {
//                 &self.data_type
//             }
//         }
//     };
// }

#[derive(Clone)]
pub struct Node<'a> {
    element_ref: ElementRef<'a>,
}

impl<'a> Node<'a> {
    fn new(element_ref: ElementRef<'a>) -> Self {
        Node { element_ref }
    }

    fn at_css(&self, selector: String) -> Self {
        let element_ref = self
            .element_ref
            .select(&Selector::parse(&selector).unwrap())
            .next()
            .unwrap();

        Self::new(element_ref)
    }

    fn text(&self) -> &str {
        self.element_ref.text().next().unwrap()
    }
}

wrappable_struct!(Node<'static>, NodeWrapper, NODE_WRAPPER);

module!(NokogiriRust);

module!(HTML);

methods! {
    HTML,
    _itself,

    fn parse(html: RString) -> RubyNode {
        let html = html
            .map_err(VM::raise_ex)
            .unwrap()
            .to_string();

        let doc = &Html::parse_document(&html);
        let doc_root = doc.root_element();

        let node = Node::new(doc_root);

        Module::from_existing("NokogiriRust").get_nested_class("RubyNode").wrap_data(node, &*NODE_WRAPPER)
    }
}

class!(RubyNode);

methods! {
    RubyNode,
    itself,

    fn ruby_at_css(selector: RString) -> RubyNode {
        let selector = selector
            .map_err(VM::raise_ex)
            .unwrap()
            .to_string();

        let node = itself.get_data(&*NODE_WRAPPER).at_css(selector);

        Class::from_existing("RubyNode").wrap_data(node, &*NODE_WRAPPER)
    },

    fn ruby_text() -> RString {
        let text = itself.get_data(&*NODE_WRAPPER).text();

        RString::new_utf8(text)
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_nokogiri_rust() {
    Module::new("NokogiriRust").define(|module| {
        module.define_nested_module("HTML").define(|module| {
            module.def_self("parse", parse);
        });

        let data_class = Class::from_existing("Object");
        module
            .define_nested_class("RubyNode", Some(&data_class))
            .define(|class| {
                class.def("at_css", ruby_at_css);
                class.def("text", ruby_text);
            });
    });
}

#[test]
fn test_at_css() {
    use std::fs;

    let big_shopping_html = fs::read_to_string("./spec/fixtures/big_shopping.html")
        .expect("Something went wrong reading the file");

    let doc = &Html::parse_document(&big_shopping_html);
    let doc_root = doc.root_element();
    let node = Node::new(doc_root);

    let selector = String::from(".eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe");
    let node = node.at_css(selector);
    let title = node.text();

    assert_eq!(title, "HP Omen by Obelisk Gaming Desktop Computer, 9th Generation Intel Core i9-9900K Processor, Nvidia GeForce RTX 2080 Super 8 GB, HyperX 32 GB Ram, 1 TB");
}
