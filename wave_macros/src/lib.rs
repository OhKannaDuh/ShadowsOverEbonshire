use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Ident, Path,
};

#[derive(Default, Clone)]
struct Sides {
    north: Option<syn::Expr>,
    south: Option<syn::Expr>,
    east:  Option<syn::Expr>,
    west:  Option<syn::Expr>,
}

fn take_sockets(attrs: &[Attribute]) -> syn::Result<Sides> {
    let mut sides = Sides::default();
    for a in attrs {
        let name = a.path().to_token_stream().to_string();
        if !name.starts_with("socket_") { continue; }
        let expr: syn::Expr = a.parse_args()?; // expect an array literal of (SocketType, u32)
        match name.as_str() {
            "socket_north"      => sides.north = Some(expr),
            "socket_south"      => sides.south = Some(expr),
            "socket_east"       => sides.east  = Some(expr),
            "socket_west"       => sides.west  = Some(expr),
            "socket_vertical"   => { sides.north = Some(expr.clone()); sides.south = Some(expr); }
            "socket_horizontal" => { sides.east  = Some(expr.clone()); sides.west  = Some(expr); }
            other => return Err(syn::Error::new(a.span(), format!("unknown attribute `{}`", other))),
        }
    }
    Ok(sides)
}

fn parse_socket_type(attrs: &[Attribute]) -> syn::Result<Path> {
    for a in attrs {
        if a.path().is_ident("wave_socket") {
            let ty: Path = a.parse_args()?;
            return Ok(ty);
        }
    }
    Err(syn::Error::new_spanned(
        quote! { #[wave_socket(...)] },
        "missing `#[wave_socket(SocketType)]` on the enum",
    ))
}

#[proc_macro_derive(
    WaveTiles,
    attributes(
        wave_socket,
        socket_north, socket_south, socket_east, socket_west,
        socket_vertical, socket_horizontal
    )
)]
pub fn derive_wave_tiles(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident.clone();

    let socket_ty = match parse_socket_type(&input.attrs) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    let data_enum = match input.data {
        Data::Enum(e) => e,
        _ => return syn::Error::new(input.span(), "WaveTiles can only be derived for enums")
                .to_compile_error().into(),
    };

    let mut arms = Vec::new();
    let mut all_variants = Vec::new();

    for variant in &data_enum.variants {
        let v_ident: &Ident = &variant.ident;
        all_variants.push(quote!(Self::#v_ident));

        let sides = match take_sockets(&variant.attrs) {
            Ok(s) => s,
            Err(e) => return e.to_compile_error().into(),
        };

        let none = quote!(&[] as &'static [(#socket_ty, u32)]);
        let north = sides.north.map(|e| quote!(&#e as &'static [(#socket_ty, u32)])).unwrap_or_else(|| none.clone());
        let south = sides.south.map(|e| quote!(&#e as &'static [(#socket_ty, u32)])).unwrap_or_else(|| none.clone());
        let east  = sides.east .map(|e| quote!(&#e as &'static [(#socket_ty, u32)])).unwrap_or_else(|| none.clone());
        let west  = sides.west .map(|e| quote!(&#e as &'static [(#socket_ty, u32)])).unwrap_or_else(|| none.clone());

        arms.push(quote! {
            (Self::#v_ident, ::wave::Direction::North) => #north,
            (Self::#v_ident, ::wave::Direction::South) => #south,
            (Self::#v_ident, ::wave::Direction::East)  => #east,
            (Self::#v_ident, ::wave::Direction::West)  => #west,
        });
    }

    let gen = quote! {
        impl ::wave::Tile for #enum_ident {
            fn all() -> &'static [Self] { &[ #( #all_variants ),* ] }
        }

        impl ::wave::HasSockets<#socket_ty> for #enum_ident {
            fn sockets(&self, dir: ::wave::Direction) -> &'static [(#socket_ty, u32)] {
                match (self, dir) {
                    #( #arms )*
                }
            }
        }
    };

    gen.into()
}
