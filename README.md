# Raytracer

A simple raytracer written in Rust.

![Rendered scene](img.png)

## Features

- Diffuse (matte) materials
- Metal (reflective) materials with configurable fuzziness
- Emissive materials (light sources)
- Antialiasing via multisampling
- Parallel rendering with [rayon](https://github.com/rayon-rs/rayon)

## Usage

```bash
cargo run --release > img.ppm
```

Convert to PNG (requires ffmpeg or ImageMagick):

```bash
ffmpeg -i img.ppm img.png
# or
convert img.ppm img.png
```

## Configuration

Edit `main.rs` to adjust:

- `image_width` - output resolution
- `samples_per_pixel` - quality vs speed (more = smoother, slower)
- `depth` - max ray bounces
- `objects` - scene geometry and materials

## License

MIT
