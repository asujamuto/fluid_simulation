
use crate::vertex::Vertex;

pub struct Point{
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub mass: f32,
    pub r: f32,
}

impl Point
{
    pub fn new(x: f32, y: f32) -> Self
    {
        Point{x, y, vx: 0.0, vy: 0.0, mass: 1.0, r: 0.02}
    }

    pub fn get_shape(&mut self) -> Vec<Vertex>
    {
        let mut circle = vec![];

        // Generate Circle Vector of Points
        for i in 0..100
        {
            let theta = self.r * std::f32::consts::PI * (i as f32);
            let x = self.r * theta.cos();
            let y = self.r * theta.sin();
            let v: Vertex =  Vertex { position: [x, y] };
            &circle.push(v);
        }

        circle
    }

    pub fn update_position(&mut self, dt: f32)
    {
        self.x += self.vx * dt;
        self.y += (self.vy * dt);

    }

}