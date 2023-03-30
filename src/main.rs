#[derive(Clone)]
struct Complex {
    real: f64,
    imag: f64,
}
struct Dimensions {
    x: i32,
    y: i32,
}

impl Complex {
    fn add(&self, new:&Complex) -> Complex {
        Complex{
            real:self.real + new.real,
            imag:self.imag + new.imag,
        }
    }
    fn square(&self) -> Complex {
        Complex{
            real:self.real * self.real - self.imag * self.imag,
            imag:self.real * self.imag * 2.0,
        }
    }
    fn magnitude(&self) -> f64 {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }
}

fn mandelbrot(x:f64,y:f64) -> usize {
    let mut i = 0;
    let c = Complex{real:x,imag:y};
    let mut z = c.clone();
    while i < 207*2 && z.magnitude() < 3.0 {
        z = z.square()
            .add(&c);
        i += 1
    }
    i
}

fn pixel_to_point(x: &i32, y: &i32, dims: &Dimensions) -> (f64, f64) {
    let point_x = (x.clone() as f64 / (dims.x as f64 / 3.0)) - 2.25;
    let point_y = (y.clone() as f64 / (dims.y as f64 / 3.0)) - 1.5;
    (point_x,point_y)
}
fn main() {
    let palette = ["Æ","Ñ","Ê","Œ","Ø","M","É","Ë","È","Ã","Â","W","Q","B","Å","æ","#","N","Á","þ","E","Ä","À","H","K","R","Ž","œ","X","g","Ð","ê","q","Û","Š","Õ","Ô","A","€","ß","p","m","ã","â","G","¶","ø","ð","é","8","Ú","Ü","$","ë","d","Ù","ý","è","Ó","Þ","Ö","å","ÿ","Ò","b","¥","F","D","ñ","á","Z","P","ä","š","Ç","à","h","û","§","Ý","k","Ÿ","®","S","9","ž","U","T","e","6","µ","O","y","x","Î","¾","f","4","õ","5","ô","ú","&","a","ü","™","2","ù","ç","w","©","Y","£","0","V","Í","L","±","3","Ï","Ì","ó","C","@","n","ö","ò","s","¢","u","‰","½","¼","‡","z","J","ƒ","%","¤","I","t","o","c","î","r","j","v","1","l","í","=","ï","ì","<",">","i","7","†","[","¿","?","×","}","*","{","+","(",")","/","»","«","•","¬","|","!","¡","÷","¦","¯","-","^","ª","„","~","³","º","²","-","°","­","¹","‹","›",";",":","’","‘","‚","’","˜","ˆ","¸","…","·","¨","´","`"," "];
    let dims = Dimensions{x:1250,y:600};
    let mut y = 0;
    while y < dims.y {

        let mut x = 0;
        while x < dims.x {
            let (point_x, point_y) = pixel_to_point(&x, &y, &dims);
            let z = mandelbrot(point_x, point_y);
            print!("{}", palette[z/2]);
            x += 1;
        }
        println!("");
        y += 1;
    }
    std::thread::sleep(core::time::Duration::from_secs(100));
}
