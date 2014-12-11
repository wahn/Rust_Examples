use std::num;


struct Vector {
    x:f32,
    y:f32,
    z:f32
}

impl Vector {
    fn new(x:f32,y:f32,z:f32) -> Vector {
        Vector { x:x, y:y, z:z }
    }
    fn scale(&self, s:f32)    -> Vector  { Vector { x:self.x*s, y:self.y*s, z:self.z*s } }
    fn plus(&self, b:Vector)  -> Vector  { Vector::new(self.x+b.x, self.y+b.y, self.z+b.z) }
    fn minus(&self, b:Vector) -> Vector  { Vector::new(self.x-b.x, self.y-b.y, self.z-b.z) }
    fn dot(&self, b:Vector)   -> f32     { self.x*b.x + self.y*b.y + self.z*b.z }
    fn magnitude(&self)       -> f32     { (self.dot(*self)).sqrt() }
    fn normalize(&self)       -> Vector  { self.scale(1.0/self.magnitude())  }
}

struct Ray {
    orig:Vector,
    dir:Vector,
}

struct Color {
    r:f32,
    g:f32,
    b:f32,
}

impl Color {
    fn scale (&self, s:f32) -> Color {
        Color { r: self.r*s, g:self.g*s, b:self.b*s }
    }
    fn plus (&self, b:Color) -> Color {
        Color { r: self.r + b.r, g: self.g + b.g, b: self.b + b.b }
    }
}

struct Sphere {
    center:Vector,
    radius:f32,
    color: Color,
}

impl Sphere {
    fn get_normal(&self, pt:Vector) -> Vector {
        return pt.minus(self.center).normalize();
    }
}

struct Light {
    position: Vector,
    color: Color,
}



static WHITE:Color  = Color { r:1.0, g:1.0, b:1.0};
static RED:Color    = Color { r:1.0, g:0.0, b:0.0};
static GREEN:Color  = Color { r:0.0, g:1.0, b:0.0};
static BLUE:Color   = Color { r:0.0, g:0.0, b:1.0};

static LIGHT1:Light = Light {
    position: Vector { x: 0.7, y: -1.0, z: 1.7} ,
    color: Color { r:1.0, g:1.0, b:1.0} ,
};


fn main() {
    println!("Hello, worlds!");
    let lut = vec!(".","-","+","*","X","M");

    let w = 20*4i;
    let h = 10*4i;

    let scene = vec!(
        Sphere{ center: Vector::new(-1.0, 0.0, 3.0), radius: 0.3, color: RED },
        Sphere{ center: Vector::new( 0.0, 0.0, 3.0), radius: 0.8, color: GREEN },
        Sphere{ center: Vector::new( 1.0, 0.0, 3.0), radius: 0.3, color: BLUE }
        );


    for j in range(0,h) {
        println!("--");
        for i in range(0,w) {
            //let tMax = 10000f32;
            let fw:f32 = w as f32;
            let fi:f32 = i as f32;
            let fj:f32 = j as f32;
            let fh:f32 = h as f32;

            let ray = Ray {
                orig: Vector::new(0.0,0.0,0.0),
                dir:  Vector::new((fi-fw/2.0)/fw, (fj-fh/2.0)/fh,1.0).normalize(),
            };

            let mut objHitObj:Option<(Sphere,f32)> = None;

            for obj in scene.iter() {
                let ret = intersect_sphere(ray, obj.center, obj.radius);
                if ret.hit {
                    objHitObj = Some((*obj,ret.tval));
                }
            }
            let pixel = match objHitObj {
                Some((obj,tval)) => lut[shade_pixel(ray,obj,tval)],
                None             => " "
            };

            print!("{}",pixel);
        }
    }

    println!("we are done!");
}


fn shade_pixel(ray:Ray, obj:Sphere, tval:f32) -> uint {
    let pi = ray.orig.plus(ray.dir.scale(tval));
    let color = diffuse_shading(pi, obj, LIGHT1);
    let col = (color.r + color.g + color.b) / 3.0;
    (col * 6.0) as uint
}

struct HitPoint {
    hit:bool,
    tval:f32,
}

fn intersect_sphere(ray:Ray, center:Vector, radius:f32) -> HitPoint {
    let l = center.minus(ray.orig);
    let tca = l.dot(ray.dir);
    if tca < 0.0 {
        return HitPoint { hit:false, tval:-1.0 };
    }
    let d2 = l.dot(l) - tca*tca;
    let r2 = radius*radius;
    if d2 > r2 {
        return HitPoint { hit: false, tval:-1.0 };
    }
    let thc = (r2-d2).sqrt();
    let t0 = tca-thc;
    //let t1 = tca+thc;
    if t0 > 10000.0 {
        return HitPoint { hit: false, tval: -1.0 };
    }
    return HitPoint { hit: true, tval: t0}

}

fn clamp(x:f32,a:f32,b:f32) -> f32{
    if x < a { return a;  }
    if x > b { return b; }
    return x;
}

fn diffuse_shading(pi:Vector, obj:Sphere, light:Light) -> Color{
    let n = obj.get_normal(pi);
    let lam1 = light.position.minus(pi).normalize().dot(n);
    let lam2 = clamp(lam1,0.0,1.0);
    light.color.scale(lam2*0.5).plus(obj.color.scale(0.3))
}
