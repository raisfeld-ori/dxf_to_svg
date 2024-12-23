# dxf_to_svg

A simple rust Package for converting a Vector of the dxf::entities::Entity type into an SVG string.

this library is so simple, that it's just one function. simply run

```rs
dxf_to_svg::dxf_to_svg(vec![])
```

And you will be getting an empty SVG. to include more entitites, simply add them to the vector.

## will I continue working on this?

this library is part of a small (closed source) project of mine. I will probably update it if I see I need to for the closed source project, but other than that, not really...

However, I will accept pull requests, so feel free to add support for other entities.
