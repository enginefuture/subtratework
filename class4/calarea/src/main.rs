use std::fmt;
trait CalArea{
    fn area(&self) -> f64;
}
#[derive(Debug)]
struct Circle{
    r: f64
}
impl CalArea for Circle{
    fn area(&self) -> f64{
        std::f64::consts::PI*(self.r*self.r)
    }
}
#[derive(Debug)]
struct Triangle{
    a: f64, b: f64,c: f64
}
impl CalArea for Triangle{
    fn area(&self) -> f64{
        let p = (self.a+self.b+self.c)/2.0;
        (p*(p-self.a)*(p-self.b)*(p-self.c)).sqrt()
    }
}
#[derive(Debug)]
struct Rectangle{
    a: f64, b: f64,
}
impl CalArea for Rectangle{
    fn area(&self) -> f64{
        self.a*self.b
    }
}

fn printarea<T: CalArea+fmt::Debug>(s :T) {
    let area = s.area();
    println!("{:?} area is {}",s,area);
}

fn main() {
    let circle = Circle{r : 12.0};
    let rectangle = Rectangle{a:3.0,b:4.0};
    let triangle = Triangle{a:3.0,b:4.0,c:5.0};
    printarea(circle);
    printarea(triangle);
    printarea(rectangle);
}
