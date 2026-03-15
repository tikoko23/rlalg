# RLalg
A simple, (mostly) minimal linear algebra library for basic vector / matrix math.

# Example
```rust
use rlalg::v3f;

struct Box {
    velocity: v3f,
    position: v3f,
    size: v3f,
}

impl Box {
    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}

```