# Resources

Resources are used to store game data that is not associated to any particular entity. Resource
Views can be borrowed from a Resources collection, just like Component Views can be borrowed from a
World.

```rust, ignore
use sparsey::prelude::*;

struct Score(f32);
struct ScoreMultiplier(f32);

fn main() {
    let mut resources = Resources::default();
    resources.insert(Score(0.0));
    resources.insert(ScoreMultiplier(1.5));

    {
        // Resources Views can be borrowed mutably or immtably.
        let mut score: ResMut<Score> = resources.borrow_mut::<Score>().unwrap();
        let score_multiplier: Res<ScoreMultipler> = resources.borrow::<ScoreMultipler>().unwrap();

        for i in 0..5 {
            score.0 += i as f32 * score_multiplier.0;
        }
    }

    let _: Option<Score> = resources.remove::<Score>();
    let _: Option<ScoreMultiplier> = resources.remove::<ScoreMultiplier>();
}
```
