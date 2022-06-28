extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};

#[proc_macro_derive(Queryable)]
pub fn macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_macro(&ast)
}

fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("Only support Struct")
    };

    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = f.ty.clone();
        let name_as_str = format!("{}", &name.clone().unwrap());
        let name_as_str = name_as_str.as_str();
        let is_i16 = &f.ty.to_token_stream().to_string().contains("i16");
        let is_i32 = &f.ty.to_token_stream().to_string().contains("i32");
        let is_i64 = &f.ty.to_token_stream().to_string().contains("i64");
        let is_bool = &f.ty.to_token_stream().to_string().contains("bool");
        let is_f32 = &f.ty.to_token_stream().to_string().contains("f32");
        let is_f64 = &f.ty.to_token_stream().to_string().contains("f64");
        let is_numeric = &f.ty.to_token_stream().to_string().contains("Decimal");
        let is_naive_date = &f.ty.to_token_stream().to_string().contains("NaiveDate");
        let is_naive_date_time = &f.ty.to_token_stream().to_string().contains("NaiveDateTime");
        let is_option = &f.ty.to_token_stream().to_string().contains("Option");

        if is_option == &true {
            if is_numeric == &true {
                return quote! {#name:  row.get::<Decimal, _>(#name_as_str)};
            }

            if is_i16 == &true {
                return quote! {#name:  row.get::<i16, _>(#name_as_str)};
            }

            if is_bool == &true {
                return quote! {#name:  row.get::<bool, _>(#name_as_str)};
            }

            if is_i32 == &true {
                return quote! {#name:  row.get::<i32, _>(#name_as_str) };
            }

            if is_i64 == &true {
                return quote! {#name:  row.get::<i64, _>(#name_as_str)};
            }

            if is_f32 == &true {
                return quote! {#name:  row.get::<f32, _>(#name_as_str) };
            }

            if is_f64 == &true {
                return quote! {#name:  row.get::<f64, _>(#name_as_str)};
            }

            if is_naive_date == &true {
                if is_naive_date_time == &true {
                    return quote! {#name:  row.get::<NaiveDateTime, _>(#name_as_str) };
                } else {
                    return quote! {#name:  row.get::<NaiveDate, _>(#name_as_str)};
                }
            }

            quote! {#name:  row.get::<&str, _>(#name_as_str).map(|s| s.to_string()) }
        } else {
            if is_numeric == &true {
                return quote! {#name:  row.get::<#ty, _>(#name_as_str).unwrap()};
            }

            if is_i16 == &true {
                return quote! {#name:  row.get::<#ty, _>(#name_as_str).unwrap() };
            }

            if is_i32 == &true {
                return quote! {#name:  row.get::<#ty, _>(#name_as_str).unwrap() };
            }
            if is_i64 == &true {
                return quote! {#name:  row.get::<#ty, _>(#name_as_str).unwrap() };
            }

            if is_bool == &true {
                return quote! {#name:  row.get::<#ty, _>(#name_as_str)};
            }

            if is_f32 == &true {
                return quote! {#name:  row.get::<#ty, _>(#name_as_str) };
            }

            if is_f64 == &true {
                return quote! {#name:  row.get::<#ty, _>(#name_as_str)};
            }

            if is_naive_date == &true {
                return quote! {#name:  row.get::<#ty, _>(#name_as_str).unwrap() };
            }
            quote! {#name:  row.get::<&str, _>(#name_as_str).map(|s| s.to_string()).unwrap() }
        }
    });

    let expanded = quote! {
        impl ::query_tiberius::query_tiberius::QueryTiberius for #struct_name {
            type Output = Self;
            fn convert_from_sql(row: &::tiberius::Row) -> Self {

                Self{
                    #(
                        #optionized,
                    )*
                }

            }
        }
    };
    expanded.into()
}
