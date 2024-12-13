//! Generate functions for rust and wgsl similar to blender's "Color Ramp" node.
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
mod parse;

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
/// 
/// # Choose a spline
/// 
/// The default linear spline is continuous but not smooth.
/// 
/// `steps` spline is not continuous.
/// 
/// `ease` spline is smooth but the tangent of anchors are alway horizontal.
#[proc_macro]
#[proc_macro_error]
pub fn ramp(token_stream: TokenStream) -> TokenStream {
    match parse::ramp(token_stream.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => abort!(e.span, "{}", e.string),
    }
}
