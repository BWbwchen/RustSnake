# Rust Snake Game
A snake game in rust with well-structed code.

## Architecture of code
The game is mainly controlled by struct **Game** , and based on struct **Block**.

```rust
struct Game {
    snake: snake::Snake,
    food: food::Food,
    background: background::BackGround,
}
```

```rust
struct Snake {
    body: VecDeque<Block>,
    direction: Direction,
    add_len: bool,
}
```

```rust
struct Food {
    f: Block,
}
```

```rust
struct Block {
    x: usize,
    y: usize,
}
```

```rust
struct BackGround {
    map: [[Block; WIDTH]; HEIGHT],
}
```
