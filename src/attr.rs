use syn;

type Ctxt = ();

#[derive(Debug, Default)]
pub struct Input {
    pub debug: Option<InputDebug>,
}

#[derive(Debug, Default)]
pub struct Field {
    pub debug: Option<FieldDebug>,
}

#[derive(Debug)]
pub struct InputDebug {
    bounds: Vec<syn::WherePredicate>,
    pub transparent: bool,
}

#[derive(Debug)]
pub struct FieldDebug {
    bounds: Vec<syn::WherePredicate>,
    pub ignore: bool,
}

impl Input {
    pub fn from_ast(cx: &Ctxt, attrs: &[syn::Attribute]) -> Input {
        let mut input = Input::default();

        for meta_items in attrs.iter().filter_map(derivative_attribute) {
            for meta_item in meta_items {
                match *meta_item {
                    // Parse `#[derivative(Debug)]`
                    syn::MetaItem::Word(ref name) if name == "Debug" => {
                        input.debug = Some(
                            InputDebug {
                                bounds: Vec::new(),
                                transparent: false,
                            }
                        );
                    }
                    // Parse `#[derivative(Debug="transparent")]`
                    syn::MetaItem::NameValue(ref name, ref value) if name == "Debug" => {
                        input.debug = Some(
                            InputDebug {
                                bounds: Vec::new(),
                                transparent: true, // TODO: check the value
                            }
                        );
                    }
                    _ => {
                        panic!("Unknown attribute");
                    }
                }
            }
        }

        input
    }
}

impl Field {
    pub fn from_ast(cx: &Ctxt, field: &syn::Field) -> Field {
        let mut out = Field::default();

        for meta_items in field.attrs.iter().filter_map(derivative_attribute) {
            for meta_item in meta_items {
                match *meta_item {
                    // Parse `#[derivative(Debug="ignore")]`
                    syn::MetaItem::NameValue(ref name, ref value) if name == "Debug" => {
                        out.debug = Some(
                            FieldDebug {
                                bounds: Vec::new(),
                                ignore: true, // TODO: check the value
                            }
                        );
                    }
                    _ => {
                        panic!("Unknown attribute");
                    }
                }
            }
        }

        out
    }
}

fn derivative_attribute(attr: &syn::Attribute) -> Option<&[syn::MetaItem]> {
    match attr.value {
        syn::MetaItem::List(ref name, ref mis) if name == "derivative" => {
            Some(mis)
        }
        syn::MetaItem::Word(..) |
        syn::MetaItem::NameValue(..) |
        syn::MetaItem::List(..) => None,
    }
}
