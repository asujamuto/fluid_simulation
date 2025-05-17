mod point;
mod vertex;

use std::iter::zip;
use std::path::absolute;
use glium::{implement_vertex, uniform, Surface};
use crate::point::Point;
use rand::prelude::*;
use rand::TryRngCore;

fn check_window_collision(point: &mut Point)
{
    if point.x + point.r > 1.0 {
        point.x = 1.0 - point.r;
        point.vx *= -1.0;
    } else if point.x - point.r < -1.0 {
        point.x = -1.0 + point.r;
        point.vx *= -1.0;
    }

    if point.y - point.r < -1.0 {
        point.y = -1.0 + point.r;
        // point.vy *= -0.8;
        point.vy = 0.0;
    } else if point.y + point.r > 1.0 {
        point.y = 1.0 - point.r;
        point.vy *= -0.8;
        // point.vy = 0.0;
    }
}

fn check_collision(p1: &mut Point, p2: &mut Point) -> bool
{
    let g = 9.81;
    if ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)) <= (p1.r + p2.r).powf(2.0)
    {
        // println!("Balls colliding p1.x: {} p2.x: {} velocity p1: {} velocity p2: {}", p1.x, p2.x, p1.vy, p2.vy);
        let higher_point;
        let lower_point;
        if p1.y <= p2.y {
            higher_point = p2;
            lower_point = p1;
        }
        else {
            higher_point = p1;
            lower_point = p2;
        }

        // Problem jest w tej linijce
        higher_point.y = lower_point.y + (lower_point.r * 2.0);
        higher_point.vy *= -(higher_point.y/10.0)*g;

        true
    }
    else if ((p1.x  - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)) > (p1.r  + p2.r).powf(2.0)
    {
        false
    }
    else
    {
        false
    }
}

fn main() {


    let g = 9.81;
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(800, 800)
        .with_title("Fluid Simulation")
        .build(&event_loop);


    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    // Fragment of GLSL code
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        uniform float x_off;
        uniform float y_off;

        void main() {
            vec2 pos = position;
            pos.x += x_off;
            pos.y += y_off;
            gl_Position = vec4(pos, 0.0, 1.0);
        }"#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }"#;
    let fragment_shader_src_hitbox = r#"
        #version 140

        out vec4 color;


        void main() {
            color = vec4(0.0, 1.0, 0.0, 0.3);
        }"#;

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src, None).unwrap();
    let program_hitbox = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src_hitbox, None).unwrap();



    let mut t: f32 = 0.0;
    let mut points = vec![Point::new(0.0, 0.0)];
    for _ in 0..200
    {
        let mut rng = rand::rng();
        points.push(Point::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)));
        // points.push(Point::new(1.0, rng.gen_range(-1.0..1.0)));
    }

    let mut points_uniforms = Vec::new();
    let mut points_uniforms_hitbox = Vec::new();
    let mut circle_buffers = Vec::new();
    let mut circle_buffers_hitbox = Vec::new();


    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } =>
                match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::RedrawRequested => {

                    t += 0.00002;

                    points_uniforms.clear();
                    circle_buffers.clear();
                    points_uniforms_hitbox.clear();
                    circle_buffers_hitbox.clear();

                    for mut p in points.iter_mut()
                    {
                        // Tutaj zamiast update'atować pozycję,
                        // powinno się
                        // 1. policzyć siły na podstawie hitboxów
                        // 2.
                        p.update_position(t);
                        check_window_collision(&mut p);
                        points_uniforms.push(uniform! {x_off: p.x, y_off: p.y});
                        points_uniforms_hitbox.push(uniform! {x_off: p.x, y_off: p.y});
                        circle_buffers.push(glium::VertexBuffer::new(&display, &p.get_shape()).unwrap());
                        circle_buffers_hitbox.push(glium::VertexBuffer::new(&display, &p.get_hitbox()).unwrap());
                    }


                    // WERSJA 2
                    for i in 0..points.len() {
                        for j in i+1..points.len() {
                            let (p1, p2) = {
                                //split_at_mut dzieli tablicę na dwie części
                                let (left, right) = points.split_at_mut(j);
                                (&mut left[i], &mut right[0])
                            };

                            if check_collision(p1, p2) {
                                p1.vy += g * t;
                                p2.vy += g * t;
                            }
                            else {
                                p1.vy += g * t;
                                p2.vy += g * t;
                            }

                        }
                    }

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    // for (u, cb) in zip(points_uniforms_hitbox.iter(), circle_buffers_hitbox.iter()) {
                    //     target.draw(cb, &indices, &program_hitbox, u,
                    //                 &Default::default()).unwrap();
                    // }

                    for (u, cb) in zip(points_uniforms.iter(), circle_buffers.iter()) {
                        target.draw(cb, &indices, &program, u,
                                    &Default::default()).unwrap();
                    }

                    target.finish().unwrap();

                },


                _ => (),
            },
            glium::winit::event::Event::AboutToWait =>  {
                _window.request_redraw();
            },
            _ => (),
        };
    });
}
