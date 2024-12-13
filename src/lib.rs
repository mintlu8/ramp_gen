//! Generate functions for rust and wgsl similar to blender's "Color Ramp" node.

use proc_macro2::{Group, Span, TokenStream, TokenTree};
use proc_macro::TokenStream as TokenStream1;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote};

enum Curve {
    Linear,
    Steps,
    Ease,
}

impl Curve {
    fn clamped(&self) -> bool {
        matches!(self, Curve::Steps | Curve::Ease)
    }
}

fn split2(group: Group) -> (TokenStream, TokenStream) {
    let span = group.span();
    let mut first = Vec::new();
    let mut iter = group.stream().into_iter();

    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == ',' => {
                return (first.into_iter().collect(), iter.collect());
            },
            _ => first.push(tt),
        }
    }
    abort!(span, "Expected 2 values separated by comma.")
}

fn parse_buf(
    wgsl: bool,
    x: &TokenStream,
    buf: &[(TokenStream, TokenStream)], 
    parse4: impl Fn(&TokenStream, &TokenStream, &TokenStream, &TokenStream) -> TokenStream
) -> TokenStream {
    match buf {
        [] => {
            abort!(Span::call_site(), "Expected at least one item.")
        }
        [(_, y)] => {
            quote! {#y}
        }
        [(x1, y1), (x2, y2)] => {
            parse4(x1, y1, x2, y2)
        },
        [(x1, y1), rest @ ..] => {
            // always true
            let (x2, y2) = &rest[0];
            let this = parse4(x1, y1, x2, y2);
            let rest = parse_buf(wgsl, x, rest, parse4);
            if wgsl {
                quote! {
                    select(
                        #this,
                        #rest,
                        #x >= #x2
                    )
                }
            } else {
                quote! {
                    if #x >= #x2 {#rest} else {#this}
                }
            }
            
        }

    }
}

/// Generate an expression that simulates the effect of the blender "Color Ramp" node.
/// 
/// Syntax: `ramp!(attr1 attr2 [1.0, 2.0], [3.0, 4.0])`
/// 
/// By default we generate a linear rust expression with variable `x`.
/// 
/// ```
/// # use ramp_gen::ramp;
/// let f = |x: f32| ramp!([0.0, 0.0], [1.0, 1.0], [2.0, 0.0]);
/// assert_eq!(f(1.5), 0.5);
/// 
/// // Use smoothstep to smooth the curve.
/// let f2 = |t: f32| ramp!(@t ease [0.0, 0.0], [1.0, 1.0], [2.0, 0.0]);
/// assert!(f2(0.25) < 0.25);
/// # assert!(f2(0.25) > 0.0);
/// ```
/// 
/// Where we create a curve intersecting these points `(x, y)`.
/// 
/// For rust, `x` must be a `f32`, while `y` can be a vector like `Vec2::new(1., 2.)`.
/// 
/// For wgsl, `x` is also allowed to be a vector like `vec2(1.0, 2.0)`.
/// 
/// # Attributes
/// 
/// * `@a`: Change the variable name from `x` to `a`.
/// 
/// * `clamp`: Clamp the input.
/// 
/// * `steps`: Generate segments with constant values.
/// 
/// * `ease`: Use the `smoothstep` function instead of linear interpolation.
/// 
/// * `wgsl`: Generates a wgsl function instead.
/// 
///     The result will likely not be valid in rust, `inline macro` using an editor and copy to your wgsl shaders.
/// 
/// * `str`: Convert the resulting expression into a string.
#[proc_macro]
#[proc_macro_error]
pub fn ramp(token_stream: TokenStream1) -> TokenStream1 {
    ramp2(token_stream.into()).into()
}

fn ramp2(token_stream: TokenStream) -> TokenStream {
    let mut iter = token_stream.clone().into_iter().peekable();
    let mut wgsl = false;
    let mut str = false;
    let mut clamp = false;
    let mut curve = Curve::Linear;
    let mut x = format_ident!("x");
    while let Some(tt) = iter.peek() {
        match tt {
            TokenTree::Punct(p) if p.as_char() == ',' => (),
            TokenTree::Punct(p) if p.as_char() == '@' => {
                let span = p.span();
                let _ = iter.next();
                match iter.next() {
                    Some(TokenTree::Ident(ident)) => x = ident,
                    Some(tt) => abort!(tt.span(), "Expected x variable."),
                    None => abort!(span, "Expected x variable."),
                }
                continue;
            },
            TokenTree::Ident(ident) => match ident.to_string().as_str() {
                "wgsl" => wgsl = true,
                "step" => curve = Curve::Steps,
                "steps" => curve = Curve::Steps,
                "ease" => curve = Curve::Ease,
                "str" => str = true,
                "clamp" => clamp = true,
                unknown => abort! {
                    ident.span(),
                    "Unknown attribute {}.", unknown
                },
            },
            _ => break,
        }
        
        let _ = iter.next();
    }
    let mut buffer = Vec::new();
    for tt in iter {
        match tt {
            TokenTree::Punct(p) if p.as_char() == ',' => (),
            TokenTree::Group(g) => buffer.push(split2(g)),
            _ => abort!(tt.span(), "Expected () or []."),
        }
    }
    if buffer.is_empty() {
        abort!(Span::call_site(), "Expected at least one item.")
    }
    let min = &buffer[0].0;
    let max = &buffer[buffer.len() - 1].0;
    let x = if !clamp || curve.clamped(){
        quote!(#x)
    } else if wgsl {
        quote!(clamp(#x, #min, #max))
    } else {
        quote!(#x.clamp(#min, #max))
    };
    let mut result = match curve {
        Curve::Linear => {
            parse_buf(wgsl, &x, &buffer, |x1, y1, x2, y2| {
                quote! {(#x - #x1) / (#x2 - #x1) * (#y2 - #y1) + #y1}
            })
        },
        Curve::Steps => {
            buffer.push((quote! {}, quote! {}));
            parse_buf(wgsl, &x, &buffer, |_, y1, _, _| {
                quote! {#y1}
            })
        },
        Curve::Ease => {
            let result = parse_buf(wgsl, &x, &buffer, |x1, y1, x2, y2| {
                quote! {(smoothstep(#x1, #x2, #x) * (#y2 - #y1) + #y1)}
            });
            if wgsl {
                result
            } else {
                quote! {{
                    let smoothstep = |l: f32, r: f32, x: f32| {
                        let t = ((x - l) / (r - l)).clamp(0.0, 1.0);
                        t * t * (3.0 - 2.0 * t)
                    };
                    #result
                }}
            }
        },
    };

    if str {
        let string = result.to_string();
        result = quote! {#string};
    }

    result
}

