# dxf_to_svg

A simple rust Package for converting dxf into an SVG.

## Usage

To convert a .dxf file:

```rust
fn file_to_svg() {
    let svg = dxf_file_to_svg("path/to/file.dxf", None);
}
```

To turn an existing vec of dxf::entities::Entity into an SVG:

```rust
fn vec_to_svg(vec: Vec<&Entity>){
    return dxf_to_svg(vec, None);
}
```

You can also replace the None for the second argument with ```dxf_to_svg::SvgOptions``` in order to style the SVG a bit.

- use_bounds -> to false if you don't want the bounding box to fix the screen
- padding -> the SVG's padding

## will I continue working on this?

I quit working on this project and instead started using
CloudConvert instead. Works way better, but costs some money

However, feel free to continue using this project. It's not perfect but it still works fairly well.
