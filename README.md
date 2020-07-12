# dust n rust

A 2D-platformer demo-game written on Rust and based on Amethyst game engine. Created mainly as an sandbox for learning Rust-laguage and exploring Amethyst game-engine.

![image](https://github.com/AskePit/askepit.github.io/blob/master/dustnrust_screen.png)

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
