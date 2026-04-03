This is a simple ray tracer built in Rust as a personal project to learn both Rust and computer graphics.

So far, it supports basic ray–object intersection, diffuse (Lambertian) materials, and metallic surfaces with fuzzed reflections.

Based on Ray Tracing in One Weekend by Peter Shirley     https://raytracing.github.io/books/RayTracingInOneWeekend.html

## Running

The renderer outputs a PPM image file to stdout. To build and run:

​```
cargo run --release > image.ppm
​```

The `--release` flag is recommended as it enables compiler optimizations, which makes a noticeable difference in render times. The output can be opened with any PPM-compatible viewer. [This site](https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html) works well for a quick look in the browser.