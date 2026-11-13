# CatFrame

A tiny terminal application that plays ASCII-art cat animations, frame by frame, centered in your terminal.

CatFrame embeds its animations as JSON, deserializes them with `serde`, and renders each frame with `crossterm`. No external files, no setup: run it and a cat appears.

## Features

- Frame-by-frame ASCII animations rendered in the center of the terminal
- Multiple animation variants, switchable at runtime
- Animations loaded from a single embedded JSON file via `serde_json`
- Raw terminal mode with hidden cursor for clean playback (`crossterm`)

## Installation

From crates.io:

```bash
cargo install catframe
```

From source:

```bash
git clone https://github.com/your-username/catframe.git
cd catframe
cargo run --release
```

## Usage

Just run the binary:

```bash
catframe
```

### Controls

| Key      | Action                                 |
| -------- | -------------------------------------- |
| `g`      | Switch to the `le_chonk` animation     |
| `c`      | Switch back to the `le_chat` animation |
| `Ctrl+C` | Quit                                   |

Note: the terminal must be at least as large as the animation dimensions for centering to display correctly.

## Animation format

Animations live in `ressources/LeChat-annimation.json` and follow this structure:

```json
{
  "variants": {
    "le_chat": {
      "cols": 40,
      "rows": 12,
      "frame_count": 8,
      "frames": [
        ["frame 1 line 1", "frame 1 line 2"],
        ["frame 2 line 1", "frame 2 line 2"]
      ]
    }
  }
}
```

- `variants`: map of animation name to animation data
- `cols` / `rows`: dimensions used for terminal centering
- `frame_count`: number of frames in the animation
- `frames`: list of frames, each frame being a list of lines

## Dependencies

- [`crossterm`](https://crates.io/crates/crossterm) - terminal rendering, raw mode, and input handling
- [`serde`](https://crates.io/crates/serde) / [`serde_json`](https://crates.io/crates/serde_json) - JSON deserialization
- [`clearscreen`](https://crates.io/crates/clearscreen) - cross-platform screen clearing

## License

MIT
