use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vec2<T>(pub T, pub T);

//struct Vec3<T>(T, T, T);