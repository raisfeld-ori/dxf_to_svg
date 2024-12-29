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

This is going to be the final version for now.
I plan on making a new version of this library by
converting the python package [dxf2svg](https://bitbucket.org/lukaszlaba/dxf2svg/wiki/Home) to rust code.

Once the new package is out I will mention it here.

While I won't work on this package, if you send me a pull
request or an issue on the github repo I will respond.
