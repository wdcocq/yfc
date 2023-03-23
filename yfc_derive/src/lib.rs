use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    DeriveInput, Error, GenericArgument, PathArguments, Result, Token,
};

#[rustfmt::skip::macros(quote)]
#[proc_macro_derive(Model, attributes(yfc))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    expand_model(ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

enum FieldAttr {
    Model,
    List,
    Value,
}

impl Parse for FieldAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        if ident == "model" {
            Ok(FieldAttr::Model)
        } else if ident == "value" {
            Ok(FieldAttr::Value)
        } else if ident == "list" {
            Ok(FieldAttr::List)
        } else {
            Err(input.error("Expected model, value or list"))
        }
    }
}

fn expand_model(ast: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let fields: Vec<syn::Field> = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Model can't have unnamed fields",
                ));
            }

            fields.iter().cloned().collect()
        }
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "#[derive(Model)] can only be used with struct",
            ))
        }
    };

    let mut value_names = vec![];
    let mut value_types = vec![];
    let mut value_idents = vec![];
    let mut value_idents_setter = vec![];
    let mut value_forms = vec![];

    let mut value_list_names = vec![];
    let mut value_list_types = vec![];
    let mut value_list_inner_types = vec![];
    let mut value_list_idents = vec![];
    let mut value_list_idents_setter = vec![];
    let mut value_list_forms = vec![];
    let mut value_list_elem_forms = vec![];

    let mut model_names = vec![];
    let mut model_types = vec![];
    let mut model_idents = vec![];
    let mut model_forms = vec![];

    let mut model_list_names = vec![];
    let mut model_list_types = vec![];
    let mut model_list_inner_types = vec![];
    let mut model_list_idents = vec![];
    let mut model_list_forms = vec![];
    let mut model_list_elem_forms = vec![];

    let struct_name = &ast.ident;
    let form_ext = format_ident!("{}FormExt", struct_name);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let visibility = ast.vis;

    for field in &fields {
        let field_ident = field.ident.clone().unwrap();
        let (field_type, field_inner_type) = match &field.ty {
            outer_ty @ syn::Type::Path(ref path) => {
                if let Some(last) = path.path.segments.last() {
                    if let PathArguments::AngleBracketed(generics) = &last.arguments {
                        if let Some(GenericArgument::Type(inner_ty)) = generics.args.first() {
                            (outer_ty, Some(inner_ty))
                        } else {
                            (outer_ty, None)
                        }
                    } else {
                        (outer_ty, None)
                    }
                } else {
                    (outer_ty, None)
                }
            }
            outer_ty @ _ => {
                return Err(Error::new(
                    field.span(),
                    format!(
                        "Type `{:?}` of field `{:?}` is not supported",
                        outer_ty, field_ident
                    ),
                ));
            }
        };

        let field_name = field_ident.to_string();
        let mut is_model = false;
        let mut is_list = false;

        if let Some(attr) = field.attrs.iter().find(|attr| attr.path.is_ident("yfc")) {
            let attributes = attr
                .parse_args_with(|input: ParseStream<'_>| {
                    input.parse_terminated::<FieldAttr, Token![,]>(FieldAttr::parse)
                })
                .unwrap();

            for attr in attributes.iter() {
                match attr {
                    FieldAttr::Model => is_model = true,
                    FieldAttr::List => is_list = true,
                    FieldAttr::Value => is_model = false,
                }
            }
        }

        match (is_model, is_list) {
            (true, true) => {
                let Some(field_inner_type) =  field_inner_type else{
                    panic!("A list requires an inner type");
                };
                model_list_names.push(field_name);
                model_list_types.push(field_type);
                model_list_inner_types.push(field_inner_type);
                model_list_forms.push(format_ident!("{}_form", field_ident));
                model_list_elem_forms.push(format_ident!("{}_elem_form", field_ident));
                model_list_idents.push(field_ident);
            }
            (true, false) => {
                model_names.push(field_name);
                model_types.push(field_type);
                model_forms.push(format_ident!("{}_form", field_ident));
                model_idents.push(field_ident);
            }
            (false, true) => {
                let Some(field_inner_type) =  field_inner_type else{
                    panic!("A list requires an inner type");
                };

                value_list_names.push(field_name);
                value_list_types.push(field_type);
                value_list_inner_types.push(field_inner_type);
                value_list_idents_setter.push(format_ident!("set_{}", field_ident));
                value_list_forms.push(format_ident!("{}_form", field_ident));
                value_list_elem_forms.push(format_ident!("{}_elem_form", field_ident));
                value_list_idents.push(field_ident);
            }
            (false, false) => {
                value_names.push(field_name);
                value_types.push(field_type);
                value_forms.push(format_ident!("{}_form", field_ident));
                value_idents_setter.push(format_ident!("set_{}", field_ident));
                value_idents.push(field_ident);
            }
        }
    }

    let (state_struct_name, state_struct_impl) = expand_state_struct(
        &visibility,
        &struct_name,
        &value_idents,
        &value_types,
        &value_list_idents,
        &value_list_types,
        &value_list_inner_types,
        &model_idents,
        &model_types,
        &model_list_idents,
        &model_list_types,
        &model_list_inner_types,
    );

    let (value_relations, value_relation_impls) = expand_relations(
        &struct_name,
        &state_struct_name,
        &value_types,
        &value_idents,
        false,
    );

    let (value_list_relations, value_list_relation_impls) = expand_relations(
        &struct_name,
        &state_struct_name,
        &value_list_types,
        &value_list_idents,
        false,
    );

    let (value_list_elem_relations, value_list_elem_relation_impls) = expand_relations(
        &struct_name,
        &state_struct_name,
        &value_list_inner_types,
        &value_list_idents,
        true,
    );

    let (model_relations, model_relation_impls) = expand_relations(
        &struct_name,
        &state_struct_name,
        &model_types,
        &model_idents,
        false,
    );

    let (model_list_relations, model_list_relation_impls) = expand_relations(
        &struct_name,
        &state_struct_name,
        &model_list_types,
        &model_list_idents,
        false,
    );

    let (model_list_elem_relations, model_list_elem_relation_impls) = expand_relations(
        &struct_name,
        &state_struct_name,
        &model_list_inner_types,
        &model_list_idents,
        true,
    );

    let (state_mut_struct_name, state_mut_struct_impl) = expand_state_mut_struct(
        &visibility,
        &struct_name,
        &state_struct_name,
        &value_idents,
        &value_types,
        &value_relations,
        &value_list_idents,
        &value_list_types,
        &value_list_relations,
        &value_list_elem_relations,
        &model_idents,
        &model_types,
        &model_relations,
        &model_list_idents,
        &model_list_types,
        &model_list_relations,
    );

    Ok(quote!(
        #state_struct_impl
        #state_mut_struct_impl

        #value_relation_impls
        #value_list_relation_impls
        #value_list_elem_relation_impls
        #model_relation_impls
        #model_list_relation_impls
        #model_list_elem_relation_impls

        #visibility trait #form_ext {
            #(
                fn #value_forms(&self) -> yfc::form::Form<#value_types>;
            )*
            #(
                fn #value_list_elem_forms(&self, index: usize) -> yfc::form::Form<#value_list_inner_types>;
                fn #value_list_forms(&self) -> yfc::form::Form<#value_list_types>;
            )*
            #(
                fn #model_forms(&self) -> yfc::form::Form<#model_types>;
            )*
            #(
                fn #model_list_elem_forms(&self, index: usize) -> yfc::form::Form<#model_list_inner_types>;
                fn #model_list_forms(&self) -> yfc::form::Form<#model_list_types>;
            )*
        }

        impl #form_ext for yfc::form::Form<#struct_name> {
            #(
                fn #value_forms(&self) -> yfc::form::Form<#value_types> {
                    self.seed(#value_relations)
                }
            )*
            #(
                fn #value_list_elem_forms(&self, index: usize) -> yfc::form::Form<#value_list_inner_types> {
                    self.seed(#value_list_elem_relations(index))
                }
                fn #value_list_forms(&self) -> yfc::form::Form<#value_list_types> {
                    self.seed(#value_list_relations)
                }
            )*
            #(
                fn #model_forms(&self) -> yfc::form::Form<#model_types> {
                    self.seed(#model_relations)
                }
            )*
            #(
                fn #model_list_elem_forms(&self, index: usize) -> yfc::form::Form<#model_list_inner_types> {
                    self.seed(#model_list_elem_relations(index))
                }
                fn #model_list_forms(&self) -> yfc::form::Form<#model_list_types> {
                    self.seed(#model_list_relations)
                }
            )*
        }

        impl #impl_generics yfc::form_state::StateProvider for #struct_name #ty_generics #where_clause {
            type State = #state_struct_name;
            type StateMut<'a> = #state_mut_struct_name<'a>;

            fn create_state(&self) -> Self::State {
                Self::State {
                    #(
                        #value_idents: self.#value_idents.create_state(),
                    )*
                    #(
                        #value_list_idents: self.#value_list_idents.create_state(),
                    )*
                    #(
                        #model_idents: self.#model_idents.create_state(),
                    )*
                    #(
                        #model_list_idents: self.#model_list_idents.create_state(),
                    )*
                }
            }
            fn create_state_mut<'a>(model: std::cell::RefMut<'a, Self>, state: ::std::cell::RefMut<'a, Self::State>) -> Self::StateMut<'a> {
               Self::StateMut {
                   model, state
               }
            }
        }

        impl #impl_generics yfc::model::Model for #struct_name #ty_generics #where_clause {
        }
    ))
}

fn expand_state_struct(
    visibility: &syn::Visibility,
    struct_name: &syn::Ident,
    value_idents: &[syn::Ident],
    value_types: &[&syn::Type],
    value_list_idents: &[syn::Ident],
    value_list_types: &[&syn::Type],
    value_list_inner_types: &[&syn::Type],
    model_idents: &[syn::Ident],
    model_types: &[&syn::Type],
    model_list_idents: &[syn::Ident],
    model_list_types: &[&syn::Type],
    model_list_inner_types: &[&syn::Type],
) -> (syn::Ident, proc_macro2::TokenStream) {
    let state_struct_name = format_ident!("{}State", struct_name);

    let idents = value_idents
        .iter()
        .chain(model_idents)
        .chain(value_list_idents)
        .chain(model_list_idents)
        .collect::<Vec<_>>();

    let types = value_types
        .iter()
        .chain(model_types)
        .chain(value_list_types)
        .chain(model_list_types)
        .collect::<Vec<_>>();

    let list_inner_idents = value_list_idents.iter().chain(model_list_idents);
    let list_inner_types = value_list_inner_types.iter().chain(model_list_inner_types);

    #[cfg(feature = "serde")]
    let derive_serde = quote!(#[derive(serde::Serialize, serde::Deserialize)]);
    #[cfg(not(feature = "serde"))]
    let derive_serde = quote!();

    let state_struct = quote!(
        #derive_serde
        #[derive(Default, Debug, Clone, PartialEq, Eq)]
        #visibility struct #state_struct_name {
            #(
                #visibility #idents: <#types as yfc::form_state::StateProvider>::State,
            )*
        }

        impl #state_struct_name {
            #visibility fn dirty(&self) -> bool {
                false
                #(
                    || self.#value_idents.dirty()
                )*
                #(
                    || self.#value_list_idents.iter().any(|x| x.dirty())
                )*
                #(
                    || self.#model_idents.dirty()
                )*
                #(
                    || self.#model_list_idents.iter().any(|x| x.dirty())
                )*

            }

            #(
                #visibility fn #list_inner_idents(&self, index: usize) -> &<#list_inner_types as yfc::form_state::StateProvider>::State {
                    &self.#list_inner_idents[index]
                }
            )*
        }
    );

    (state_struct_name, state_struct)
}

fn expand_state_mut_struct(
    visibility: &syn::Visibility,
    struct_name: &syn::Ident,
    state_struct_name: &syn::Ident,
    value_idents: &[syn::Ident],
    value_types: &[&syn::Type],
    value_relations: &[syn::Ident],
    value_list_idents: &[syn::Ident],
    value_list_types: &[&syn::Type],
    value_list_relations: &[syn::Ident],
    value_list_elem_relations: &[syn::Ident],
    model_idents: &[syn::Ident],
    model_types: &[&syn::Type],
    model_relations: &[syn::Ident],
    model_list_idents: &[syn::Ident],
    model_list_types: &[&syn::Type],
    model_list_relations: &[syn::Ident],
) -> (syn::Ident, proc_macro2::TokenStream) {
    let state_mut_struct_name = format_ident!("{}Mut", state_struct_name);
    let set_fn_names = value_idents.iter().map(|i| format_ident!("set_{}", i));
    let set_list_fn_names = value_list_idents.iter().map(|i| format_ident!("set_{}", i));

    let types = value_types
        .iter()
        .chain(model_types)
        .chain(value_list_types)
        .chain(model_list_types);
    let idents = value_idents
        .iter()
        .chain(model_idents)
        .chain(value_list_idents)
        .chain(model_list_idents);
    let relations = value_relations
        .iter()
        .chain(model_relations)
        .chain(value_list_relations)
        .chain(model_list_relations);

    let state_mut_struct_impl = quote!(
        #visibility struct #state_mut_struct_name<'a> {
            model: ::std::cell::RefMut<'a, #struct_name>,
            state: ::std::cell::RefMut<'a, #state_struct_name>,
        }

        impl<'a> #state_mut_struct_name<'a> {

            #(
                #visibility fn #set_fn_names<S: ::std::convert::Into<::std::rc::Rc<::std::primitive::str>>>(self, value: S) {
                    let mut state = yfc::form_state::StateMut::map(self, &#value_relations);
                    yfc::form_state::ValueStateMut::set(&mut state, value);
                }
            )*
            #(
                #visibility fn #set_list_fn_names<S: ::std::convert::Into<::std::rc::Rc<::std::primitive::str>>>(self, index: usize, value: S) {
                    let mut state = yfc::form_state::StateMut::map(self, &#value_list_elem_relations(index));
                    yfc::form_state::ValueStateMut::set(&mut state, value);
                }
            )*
            #(
                #visibility fn #idents(self) -> <#types as yfc::form_state::StateProvider>::StateMut<'a> {
                    yfc::form_state::StateMut::map(self, &#relations)
                }
            )*
        }

        impl<'a> yfc::form_state::StateMut<'a, #struct_name> for #state_mut_struct_name<'a> {
            fn split(self) -> (::std::cell::RefMut<'a, #struct_name>, ::std::cell::RefMut<'a, #state_struct_name>) {
                (self.model, self.state)
            }
        }

        impl<'a> ::std::ops::Deref for #state_mut_struct_name<'a> {
            type Target = #state_struct_name;

            fn deref(&self) -> &Self::Target {
                &self.state
            }
        }
    );

    (state_mut_struct_name, state_mut_struct_impl)
}

fn expand_relations(
    struct_name: &syn::Ident,
    state_struct_name: &syn::Ident,
    types: &[&syn::Type],
    idents: &[syn::Ident],
    list: bool,
) -> (Vec<syn::Ident>, proc_macro2::TokenStream) {
    let relations = idents
        .iter()
        .map(|i| match list {
            true => format_ident!(
                "{}{}ElemRelation",
                struct_name,
                i.to_string().to_pascal_case()
            ),
            false => format_ident!("{}{}Relation", struct_name, i.to_string().to_pascal_case()),
        })
        .collect::<Vec<_>>();

    let idents = idents.iter().map(|i| match list {
        true => quote!(#i[self.0]),
        false => quote!(#i),
    });

    let relation_structs = relations.iter().map(|r| match list {
        true => quote!(#r(usize)),
        false => quote!(#r),
    });

    let relation_structs = quote!(
        #(
            struct #relation_structs;

            impl yfc::model::ModelRelation<#struct_name, #types> for #relations {
                fn relation_model<'a>(&self, parent: &'a #struct_name) -> &'a #types {
                    &parent.#idents
                }
                fn relation_model_mut<'a>(&self, parent: &'a mut #struct_name) -> &'a mut #types {
                    &mut parent.#idents
                }
                fn relation_state<'a>(&self, parent: &'a #state_struct_name) -> &'a <#types as yfc::form_state::StateProvider>::State {
                    &parent.#idents
                }
                fn relation_state_mut<'a>(&self, parent: &'a mut #state_struct_name) -> &'a mut <#types as yfc::form_state::StateProvider>::State {
                    &mut parent.#idents
                }
            }
        )*
    );

    (relations, relation_structs)
}
