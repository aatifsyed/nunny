use clap::{Parser, ValueEnum};
use owo_colors::OwoColorize as _;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use scraper::{Html, Selector};
use syn::{
    parse::{Parse, ParseStream, Parser as _},
    punctuated::Punctuated,
    visit_mut::{self, VisitMut},
    Generics, Ident, Lifetime, Path, Token, Type, TypePath,
};

#[derive(Parser)]
struct Args {
    #[arg(
        long,
        default_value = "https://doc.rust-lang.org/1.77.2/std/convert/trait.From.html"
    )]
    url: String,
    #[arg(long, default_value = "section.impl > .code-header")]
    selector: String,
    #[arg(long, default_value = "from")]
    method: String,
    #[arg(long, default_value = "test")]
    out: Out,
}

fn main() -> anyhow::Result<()> {
    let Args {
        url,
        selector,
        method,
        out,
    } = Args::parse();
    let method = Ident::parse.parse_str(&method)?;
    let html = ureq::get(&url).call()?.into_string()?;
    let html = Html::parse_document(&html);
    for selected in
        html.select(&Selector::parse(&selector).map_err(|it| anyhow::Error::msg(it.to_string()))?)
    {
        let text = selected.text().collect::<Vec<_>>().join(" ");
        eprintln!("{}", text.dimmed());

        match syn::parse_str::<Impl>(&text).map(|it| select(it, &method, out)) {
            Ok(Some(it)) => {
                let rendered = prettyplease::unparse(&syn::File {
                    shebang: None,
                    attrs: vec![],
                    items: vec![syn::parse_quote! {
                        #it
                    }],
                });
                println!("{}", rendered.green())
            }
            Ok(None) => eprintln!("{}", "skip".yellow()),
            Err(e) => {
                let e = syn_miette::Error::new(e, text).render();
                eprintln!("{}", e.red());
            }
        }
    }
    Ok(())
}

#[derive(ValueEnum, Clone, Copy)]
enum Out {
    Test,
    Impl,
}

/// Change the types:
/// - [T] -> Slice<T>
/// - [T; N] -> Array<N, T>
///
/// Select `impls` with any of the following types:
/// - Slice
/// - Array
/// - Vec
///
/// Select impls which fill the following:
/// - impl _<$src> for $dst
///
///
fn select(mut impl_: Impl, method: &Ident, out: Out) -> Option<TokenStream> {
    struct Visitor {
        include: bool,
    }
    impl VisitMut for Visitor {
        fn visit_type_mut(&mut self, outer: &mut syn::Type) {
            match outer {
                Type::Array(inner) => {
                    visit_mut::visit_type_array_mut(self, inner);
                    let len = &inner.len;
                    let elem = &inner.elem;
                    *outer = syn::parse_quote! {
                        Array<#len, #elem>
                    };
                    self.include = true;
                }
                Type::BareFn(inner) => visit_mut::visit_type_bare_fn_mut(self, inner),
                Type::Group(inner) => visit_mut::visit_type_group_mut(self, inner),
                Type::ImplTrait(inner) => visit_mut::visit_type_impl_trait_mut(self, inner),
                Type::Infer(inner) => visit_mut::visit_type_infer_mut(self, inner),
                Type::Macro(inner) => visit_mut::visit_type_macro_mut(self, inner),
                Type::Never(inner) => visit_mut::visit_type_never_mut(self, inner),
                Type::Paren(inner) => visit_mut::visit_type_paren_mut(self, inner),
                Type::Path(inner) => {
                    visit_mut::visit_type_path_mut(self, inner);
                    if inner.path.is_ident("Vec") {
                        self.include = true;
                    }
                }
                Type::Ptr(inner) => visit_mut::visit_type_ptr_mut(self, inner),
                Type::Reference(inner) => visit_mut::visit_type_reference_mut(self, inner),
                Type::Slice(inner) => {
                    visit_mut::visit_type_slice_mut(self, inner);
                    let elem = &inner.elem;
                    *outer = syn::parse_quote! {
                        Slice<#elem>
                    };
                    self.include = true;
                }
                Type::TraitObject(inner) => visit_mut::visit_type_trait_object_mut(self, inner),
                Type::Tuple(inner) => visit_mut::visit_type_tuple_mut(self, inner),
                Type::Verbatim(_) => {}
                other => todo!("{:?}", other),
            }
        }
    }

    let mut visitor = Visitor { include: false };

    let Impl {
        impl_token: _,
        generics,
        trait_,
        for_token: _,
        ty,
    } = &mut impl_;

    visitor.visit_generics_mut(generics);
    visitor.visit_path_mut(trait_);
    visitor.visit_type_mut(ty);

    if !visitor.include {
        return None;
    }

    let Impl {
        impl_token: _,
        generics:
            Generics {
                lt_token,
                params,
                gt_token,
                where_clause,
            },
        trait_,
        for_token: _,
        ty: dst,
    } = &impl_;

    let src = trait_.segments.last().and_then(|it| match &it.arguments {
        syn::PathArguments::None => None,
        syn::PathArguments::AngleBracketed(it) => {
            let args = it.args.iter().collect::<Vec<_>>();
            match args.as_slice() {
                [syn::GenericArgument::Type(it)] => Some(it),
                _ => None,
            }
        }
        syn::PathArguments::Parenthesized(_) => None,
    })?;

    let trait_ = trait_
        .segments
        .iter()
        .map(|it| &it.ident)
        .collect::<Punctuated<_, Token![,]>>();

    let ret = match out {
        Out::Test => quote! {
            const _: () = {
                fn _test
                    #lt_token
                    #params
                    #gt_token
                ()
                #where_clause
                {
                    <#dst as #trait_<#src>>::#method;
                }
            }
        },
        Out::Impl => quote! {
            impl #lt_token #params #gt_token
                #trait_<#src>
            for #dst
            #where_clause
            {
                fn #method(&self, other: &#src) {}
            }
        },
    };

    Some(ret)
}

#[derive(Debug)]
struct Impl {
    pub impl_token: Token![impl],
    pub generics: Generics,
    pub trait_: Path,
    pub for_token: Token![for],
    pub ty: Type,
}

impl ToTokens for Impl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            impl_token,
            generics:
                Generics {
                    lt_token,
                    params,
                    gt_token,
                    where_clause,
                },
            trait_,
            for_token,
            ty,
        } = self;
        impl_token.to_tokens(tokens);
        lt_token.to_tokens(tokens);
        params.to_tokens(tokens);
        gt_token.to_tokens(tokens);
        trait_.to_tokens(tokens);
        for_token.to_tokens(tokens);
        ty.to_tokens(tokens);
        where_clause.to_tokens(tokens);
    }
}

impl Parse for Impl {
    // Hacked together from `Parse for syn::ItemImpl`
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_token = input.parse()?;

        let has_generics = input.peek(Token![<])
            && (input.peek2(Token![>])
                || input.peek2(Token![#])
                || (input.peek2(Ident) || input.peek2(Lifetime))
                    && (input.peek3(Token![:])
                        || input.peek3(Token![,])
                        || input.peek3(Token![>])
                        || input.peek3(Token![=]))
                || input.peek2(Token![const]));
        let mut generics: Generics = if has_generics {
            input.parse()?
        } else {
            Generics::default()
        };

        let mut first_ty: Type = input.parse()?;
        let trait_;

        let for_token: Token![for] = input.parse()?;
        let mut first_ty_ref = &first_ty;
        while let Type::Group(ty) = first_ty_ref {
            first_ty_ref = &ty.elem;
        }
        if let Type::Path(TypePath { qself: None, .. }) = first_ty_ref {
            while let Type::Group(ty) = first_ty {
                first_ty = *ty.elem;
            }
            if let Type::Path(TypePath { qself: None, path }) = first_ty {
                trait_ = path;
            } else {
                unreachable!();
            }
        } else {
            return Err(syn::Error::new_spanned(first_ty_ref, "expected trait path"));
        }

        let ty = input.parse()?;
        if !input.is_empty() {
            generics.where_clause = Some(input.parse()?);
        }

        Ok(Self {
            impl_token,
            generics,
            trait_,
            for_token,
            ty,
        })
    }
}
