use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};
use std::rc::Rc;
use std::cell::RefCell;

// ----------------- 파티클 구조체 -----------------
struct Particle {
    i: i32,
    total: i32,
    angle: f64,
    radius_base: f64,
    size_base: f64,
    offset: f64,
    phase: f64,
    speed: f64,
    r: f64,
    x: f64,
    y: f64,
    size: f64,
    alpha: f64,
}

impl Particle {
    fn new(i: i32, total: i32, w: f64, h: f64) -> Self {
        let mut s = Self {
            i,
            total,
            angle: 0.0,
            radius_base: 0.0,
            size_base: 0.0,
            offset: 0.0,
            phase: 0.0,
            speed: 0.0,
            r: 0.0,
            x: 0.0,
            y: 0.0,
            size: 0.0,
            alpha: 0.0,
        };
        s.reset(w, h);
        s
    }

    fn reset(&mut self, w: f64, h: f64) {
        let r = js_sys::Math::random() * 0.4 + 0.1;
        self.angle = (self.i as f64 / self.total as f64) * std::f64::consts::PI * 2.0;
        self.radius_base = w.min(h) * (0.12 + r * 0.4);
        self.size_base = 1.0 + js_sys::Math::random() * 3.0;
        self.offset = (js_sys::Math::random() - 0.5) * 50.0;
        self.phase = js_sys::Math::random() * std::f64::consts::PI * 2.0;
        self.speed = 0.002 + js_sys::Math::random() * 0.006;
    }

    fn update(&mut self, dt: f64, t: f64, speed: f64, beat: f64, size_multiplier: f64, w: f64, h: f64) {
        let spin = speed * (0.3 + (self.i as f64 / self.total as f64));
        self.angle += spin * dt * self.speed;
        self.phase += dt * 0.004;

        let beat_val = (t * beat + self.i as f64 * 0.05).sin() * 0.5 + 0.5;
        self.r = self.radius_base * (0.6 + 0.6 * beat_val + self.offset / 1000.0);
        self.x = w / 2.0 + self.angle.cos() * self.r;
        self.y = h / 2.0 + self.angle.sin() * self.r;
        self.size = self.size_base * (0.8 + beat_val * size_multiplier);
        self.alpha = 0.35 + 0.65 * beat_val;
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d, theme_func: &dyn Fn(&Particle) -> [String; 2]) -> Result<(), JsValue> {
        ctx.begin_path();
        ctx.set_global_alpha(self.alpha.max(0.0).min(1.0));
        ctx.arc(self.x, self.y, self.size, 0.0, std::f64::consts::PI * 2.0)?;
        let grad = ctx.create_radial_gradient(self.x, self.y, self.size * 0.1, self.x, self.y, self.size * 4.0)?;
        let cols = theme_func(self);
        grad.add_color_stop(0.0, &cols[0])?;
        grad.add_color_stop(1.0, &cols[1])?;
        ctx.set_fill_style(&grad);
        ctx.fill();
        ctx.set_global_alpha(1.0);
        Ok(())
    }
}

struct Simulation {
    ctx: CanvasRenderingContext2d,
    w: f64,
    h: f64,
    particles: Vec<Particle>,
    last: f64,
    acc: f64,
    t: f64,
    paused: bool,
    verse_index: usize,
    speed: f64,
    beat: f64,
    theme: String,
}

impl Simulation {
    fn new(ctx: CanvasRenderingContext2d, w: f64, h: f64, count: i32) -> Self {
        let particles = (0..count).map(|i| Particle::new(i, count, w, h)).collect();
        Self {
            ctx,
            w,
            h,
            particles,
            last: 0.0,
            acc: 0.0,
            t: 0.0,
            paused: false,
            verse_index: 0,
            speed: 1.0,
            beat: 1.0,
            theme: "taegeuk".to_string(),
        }
    }

    fn update(&mut self, now: f64) {
        let dt = (now - self.last) * 0.001;
        self.last = now;
        if !self.paused {
            self.t += dt;
            let params = (self.speed, self.beat * 2.0, 2.2);
            for p in &mut self.particles {
                p.update(dt, self.t, params.0, params.1, params.2, self.w, self.h);
            }

            self.acc += dt;
            if self.acc > 20.0 {
                for p in &mut self.particles {
                    p.reset(self.w, self.h);
                }
                self.acc = 0.0;
            }
        }
    }

    fn render(&self) -> Result<(), JsValue> {
        self.ctx.set_fill_style_str("#04121a");
        self.ctx.fill_rect(0.0, 0.0, self.w, self.h);

        let g = self.ctx.create_radial_gradient(self.w / 2.0, self.h / 2.0, 0.0, self.w / 2.0, self.h / 2.0, self.w.max(self.h) * 0.8)?;
        g.add_color_stop(0.0, "rgba(8,24,48,0.35)")?;
        g.add_color_stop(1.0, "rgba(4,8,12,0.95)")?;
        self.ctx.set_fill_style(&g);
        self.ctx.fill_rect(0.0, 0.0, self.w, self.h);

        self.ctx.set_global_composite_operation("lighter")?;
        let theme_func = get_theme_func(&self.theme);
        for p in &self.particles {
            p.draw(&self.ctx, &theme_func)?;
        }
        self.ctx.set_global_composite_operation("source-over")?;

        let cx = self.w / 2.0;
        let cy = self.h / 2.0;
        self.ctx.save();
        self.ctx.translate(cx, cy)?;
        self.ctx.set_fill_style_str("rgba(255,255,255,0.02)");
        self.ctx.begin_path();
        self.ctx.arc(0.0, 0.0, self.w.min(self.h) * 0.12, 0.0, std::f64::consts::PI * 2.0)?;
        self.ctx.fill();

        self.ctx.set_fill_style_str("rgba(255, 230, 120, 0.95)");
        self.ctx.set_font(&format!("bold {}px Noto Sans KR, sans-serif", (self.w.min(self.h) * 0.05).floor()));
        self.ctx.set_text_align("center");
        self.ctx.set_text_baseline("middle");
        let keywords = ["一", "三", "萬"];
        let key = keywords[((self.t * 0.25) % keywords.len() as f64) as usize];
        self.ctx.fill_text(key, 0.0, 0.0)?;

        let verses = [
            "一始無始 一析三極 無盡本天 一",
            "天地人合 一體運行 萬物含生",
            "一生一成 三貫萬象 無始而有"
        ];
        self.ctx.set_font("16px Noto Sans KR, sans-serif");
        self.ctx.set_fill_style_str("rgba(220,240,255,0.78)");
        self.ctx.fill_text(verses[self.verse_index], 0.0, self.w.min(self.h) * 0.12 + 20.0)?;
        self.ctx.restore();

        Ok(())
    }
}

fn get_theme_func(name: &str) -> Box<dyn Fn(&Particle) -> [String; 2]> {
    match name {
        "taegeuk" => Box::new(|p: &Particle| {
            let a = format!("rgba({}, {}, {}, 0.9)", 30 + (p.i % 40) * 3, 120 + (p.size * 10.0).floor() as i32, 200 - (p.i % 60));
            let b = format!("rgba({}, {}, {}, 0.04)", 200 - (p.i % 80), 150 + (p.size * 2.0).floor() as i32, 80 + (p.i % 60));
            [a, b]
        }),
        "monochrome" => Box::new(|p: &Particle| {
            let c1 = 200 - (p.size * 10.0).floor() as i32;
            let a = format!("rgba({},{},{},0.95)", c1, c1, c1);
            let b = format!("rgba({},{},{},0.03)", c1, c1, c1);
            [a, b]
        }),
        "fire" => Box::new(|p: &Particle| {
            let a = format!("rgba(220, {}, 20, 0.95)", 80 + (p.i % 120));
            let b = "rgba(120,40,10,0.03)".to_string();
            [a, b]
        }),
        "water" => Box::new(|p: &Particle| {
            let a = format!("rgba(20, {}, 220, 0.95)", 150 + (p.i % 90));
            let b = "rgba(10,30,80,0.03)".to_string();
            [a, b]
        }),
        _ => Box::new(|p: &Particle| {
            let a = format!("rgba({}, {}, {}, 0.9)", 30 + (p.i % 40) * 3, 120 + (p.size * 10.0).floor() as i32, 200 - (p.i % 60));
            let b = format!("rgba({}, {}, {}, 0.04)", 200 - (p.i % 80), 150 + (p.size * 2.0).floor() as i32, 80 + (p.i % 60));
            [a, b]
        }),
    }
}

#[wasm_bindgen]
struct App {
    sim: Rc<RefCell<Simulation>>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<App, JsValue> {
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Could not get 2d context"))?
            .dyn_into()?;

        let w = canvas.width() as f64;
        let h = canvas.height() as f64;

        let count_input: HtmlInputElement = web_sys::window().unwrap().document().unwrap().get_element_by_id("count").unwrap().dyn_into()?;
        let count = count_input.value().parse().unwrap_or(180);

        let sim = Rc::new(RefCell::new(Simulation::new(ctx, w, h, count)));

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let sim_clone = sim.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |now: f64| {
            let mut sim = sim_clone.borrow_mut();
            sim.update(now);
            if let Err(err) = sim.render() {
                web_sys::console::error_1(&err);
            }
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f64)>));

        request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(App { sim })
    }

    pub fn update_count(&mut self, count: i32) {
        let mut sim = self.sim.borrow_mut();
        sim.particles = (0..count).map(|i| Particle::new(i, count, sim.w, sim.h)).collect();
    }

    pub fn update_speed(&mut self, speed: f64) {
        self.sim.borrow_mut().speed = speed;
    }

    pub fn update_beat(&mut self, beat: f64) {
        self.sim.borrow_mut().beat = beat;
    }

    pub fn update_theme(&mut self, theme: String) {
        self.sim.borrow_mut().theme = theme;
    }

    pub fn toggle_pause(&mut self) {
        let mut sim = self.sim.borrow_mut();
        sim.paused = !sim.paused;
    }

    pub fn is_paused(&self) -> bool {
        self.sim.borrow().paused
    }

    pub fn next_verse(&mut self) {
        let mut sim = self.sim.borrow_mut();
        sim.verse_index = (sim.verse_index + 1) % 3;
        for p in &mut sim.particles {
            p.radius_base *= 0.92 + js_sys::Math::random() * 0.16;
            p.phase += js_sys::Math::random() * 1.2 - 0.6;
        }
    }

    pub fn reset(&mut self, count: i32) {
        let mut sim = self.sim.borrow_mut();
        sim.particles = (0..count).map(|i| Particle::new(i, count, sim.w, sim.h)).collect();
        sim.verse_index = 0;
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
