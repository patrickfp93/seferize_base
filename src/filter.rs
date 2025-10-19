use syn::{Attribute, Item, parse_quote};
pub struct Filter;

static OCCURRENCES: [&'static str; 4] = [
    "seferize::stringify",
    "stringify",
    "seferize::ignore",
    "ignore",
];

impl Filter {
    pub fn remove_self_invocations(item: &mut Item) -> bool {
        // Se o próprio item tem ignore, sinaliza remoção
        if Self::should_remove_item(item) {
            return true; // remove este item do bloco externo
        }

        match item {
            Item::Struct(s) => {
                s.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
            }
            Item::Enum(e) => {
                e.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                /*e.variants.retain(|variant| {
                    variant.attrs.retain(|attr| !Self::is_target_macro_attribure(attr));
                    true // variantes individuais não removemos ainda
                });*/
            }
            Item::Trait(t) => {
                t.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                for item in &mut t.items {
                    match item {
                        syn::TraitItem::Fn(m) => {
                            m.attrs
                                .retain(|attr| !Self::is_target_macro_attribure(attr));
                        }
                        _ => {}
                    }
                }
            }
            Item::Impl(i) => {
                i.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                i.items.retain(|impl_item| {
                    match impl_item {
                        syn::ImplItem::Fn(m) => m
                            .attrs
                            .clone()
                            .retain(|a| !Self::is_target_macro_attribure(a)),
                        syn::ImplItem::Const(c) => c
                            .attrs
                            .clone()
                            .retain(|a| !Self::is_target_macro_attribure(a)),
                        syn::ImplItem::Type(t) => t
                            .attrs
                            .clone()
                            .retain(|a| !Self::is_target_macro_attribure(a)),
                        _ => {}
                    }
                    true
                });
            }
            Item::Mod(m) => {
                m.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                if let Some((_, items)) = &mut m.content {
                    items.retain(|sub_item| !Self::remove_self_invocations(&mut sub_item.clone()));
                    // chama recursivamente os sub-itens
                    for sub_item in items {
                        Self::remove_self_invocations(sub_item);
                    }
                }
            }
            Item::Macro(mac) => {
                let segments: Vec<String> = mac
                    .mac
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect();
                if Self::is_target_macro_from_slice(&segments) {
                    *mac = parse_quote!(); // remove a macro
                }
            }
            _ => {}
        }

        false
    }

    /// Verifica se o item possui "ignore" ou "seferize::ignore" e deve ser removido
    fn should_remove_item(item: &Item) -> bool {
        let attrs = Self::get_item_attributes(item);

        for attr in attrs {
            let path_segments = attr
                .path()
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>();
            let path = path_segments.join("::");
            if path == "ignore" || path == "seferize::ignore" {
                return true; // item inteiro deve ser removido
            }
        }

        false
    }
    /// Verifica se o path da macro é um dos alvos
    fn is_target_macro_attribure(attribute: &Attribute) -> bool {
        let path_segments = attribute
            .path()
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<String>>();
        Self::is_target_macro_from_slice(&path_segments)
    }

    fn is_target_macro_from_slice(path_segments: &[String]) -> bool {
        let path = path_segments.join("::");
        OCCURRENCES.iter().any(|&occ| occ == path)
    }

    pub fn get_item_attributes(item: &Item) -> &[Attribute] {
        match item {
            Item::Const(i) => &i.attrs,
            Item::Enum(i) => &i.attrs,
            Item::ExternCrate(i) => &i.attrs,
            Item::Fn(i) => &i.attrs,
            Item::ForeignMod(i) => &i.attrs,
            Item::Impl(i) => &i.attrs,
            Item::Macro(i) => &i.attrs,
            Item::Mod(i) => &i.attrs,
            Item::Static(i) => &i.attrs,
            Item::Struct(i) => &i.attrs,
            Item::Trait(i) => &i.attrs,
            Item::TraitAlias(i) => &i.attrs,
            Item::Type(i) => &i.attrs,
            Item::Union(i) => &i.attrs,
            Item::Use(i) => &i.attrs,
            Item::Verbatim(_) => &[],
            _ => &[],
        }
    }
}
