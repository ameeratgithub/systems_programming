/// `std::alloc` provides facilities for controlling memory allocation.
use std::{
    alloc::{GlobalAlloc, Layout, System},
    time::Instant,
};

use graphics::{
    clear,
    math::{Vec2d, add, mul_scalar},
    rectangle,
};
use piston_window::{PistonWindow, WindowSettings};
use rand::{Rng, rngs::ThreadRng, thread_rng};

/// `global_allocator` marks the `ALLOCATOR` as satisfying `GlobalAlloc` trait
#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

/// Prints time taken for each allocation to STDOUT as the program runs. This provides a fairly
/// accurate indication of the time taken for dynamic memory allocation
struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        // Defers the actual memory allocation to the system's default memory allocator
        let ptr = unsafe { System.alloc(layout) };
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        eprintln!("{bytes_requested}\t{}", time_taken.as_nanos());
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout);
        }
    }
}

/// Data used for lifetime of the program
struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

/// Defines an object in 2d space
struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    fn update(&mut self) {
        // Move the particle to next position
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        // Slow down the particle's rate of increase as it travels across the screen
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        // Makes the particle more transparent over time
        self.color[3] *= 0.995;
    }
}

impl World {
    fn new(width: f64, height: f64) -> Self {
        World {
            current_turn: 0,
            particles: Vec::new(),
            height,
            width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(&self);
            // Takes the ownership of the particle, moving its data to heap and creates
            // a reference to that data on stack
            let boxed_particle = Box::new(particle);
            // Push that reference into `self.particles`
            self.particles.push(boxed_particle);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;
            // Enumeration in separate variable for better readability
            let particle_iter = self.particles.iter().enumerate();

            for (i, particle) in particle_iter {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }

            if let Some(i) = to_delete {
                // Remove the first particle that is invisible
                self.particles.remove(i);
            } else {
                // Remove the oldest particle if there's no invisible particles
                self.particles.remove(0);
            }
        }
    }

    fn update(&mut self) {
        // Random integer between -3 and 3, inclusive
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();

        for shape in &mut self.particles {
            shape.update();
        }

        self.current_turn += 1;
    }
}
pub fn visualize() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("Particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
