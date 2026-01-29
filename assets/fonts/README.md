# Fonts

shellui uses **Roboto Regular** as the default font. `Roboto-Regular.ttf` in this directory is compiled in via `include_bytes!` and loaded on startup—no extra setup needed.

**Optional:** Call `renderer.load_font("path/to/custom.ttf")` before `run()` to use a different font, or use `renderer.set_font(font)` with bytes from `include_bytes!`.
