use raylib::prelude::*;
use rand::seq::SliceRandom;
use rand::*;

#[derive(Debug)]
pub struct ParticleSystem {
    particles: Vec<Particle>
}

#[derive(Debug)]
pub struct Particle {
    position:   Vector2,
    velocity:   Vector2,
    color:      Color,
    lifetime:   f32,
    scale:      f32
}

impl Particle {
    pub fn step(self: &mut Self, timestep: f32) {
        self.position.x = self.position.x + self.velocity.x * timestep;
        self.position.y = self.position.y + self.velocity.y * timestep;
        if self.lifetime > 0.0 {
            self.lifetime -= timestep;
        }
    }

    pub fn draw(self: &Self, draw_context: &mut RaylibDrawHandle, texture: &Texture2D) {
        if self.lifetime < 0.0 { return; }

        let destination_size = Vector2 {
            x: self.scale * texture.width  as f32, 
            y: self.scale * texture.height as f32
        };

        draw_context.draw_texture_pro(
            texture, 
            Rectangle {
                x: 0.0,
                y: 0.0,
                width:  texture.width  as f32,
                height: texture.height as f32
            },
            Rectangle {
                x: self.position.x + destination_size.x / 2.0, 
                y: self.position.y + destination_size.y / 2.0,
                width:  destination_size.x,
                height: destination_size.y
            },
            Vector2 { 
                x: destination_size.x / 2.0,
                y: destination_size.y / 2.0 
            },
            self.lifetime * 360.0,
            self.color
        );
    }
}

impl ParticleSystem {
    pub fn step(self: &mut Self, timestep: f32) {
        for particle in &mut self.particles {
            particle.step(timestep);
        }
    }

    pub fn draw(self: &Self, draw_context: &mut RaylibDrawHandle, texture: &Texture2D) {
        for particle in &self.particles {
            particle.draw(draw_context, texture);
        }
    }

    pub fn create_radial(lifetime: f32, colors: Vec<Color>, count: u8, starting_pos: Vector2, intensity: f32, scale: f32) -> Self {
        let mut particles: Vec<Particle> = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..count {
            particles.push(
                Particle {
                    position:   starting_pos,
                    velocity:   Vector2 {
                        x: rng.gen_range(-1.0..=1.0) * intensity,
                        y: rng.gen_range(-1.0..=1.0) * intensity
                    },
                    color:      colors.choose(&mut rand::thread_rng()).unwrap().clone(),
                    lifetime:   lifetime,
                    scale:      scale
                }
            );
        }

        ParticleSystem {
            particles: particles
        }
    }

    pub fn reset(self: &mut Self, lifetime: f32, starting_pos: Vector2) {
 	    let mut rng = thread_rng();
    	for particle in &mut self.particles {
    		particle.position = starting_pos;
    		particle.lifetime = lifetime;
    	}
    }
} 