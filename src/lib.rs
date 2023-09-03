mod md_about;

use std::error::Error;

#[derive(Debug,Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: String,
}

pub struct Line<'a> {
    pub p1: &'a Point,
    pub p2: &'a Point,
    pub name: String,
}

impl Point {
    pub fn out(&self) {
        print!("point[{}]: ", self.name);
        println!("{}  {}  {}", self.x, self.y, self.z);
    }
}

impl Line<'_> {
    pub fn out(&self) {
        print!("Line[{}]: point[{}]: ", self.name, self.p1.name);
        println!("{}  {}  {}", self.p1.x, self.p1.y, self.p1.z);
        print!("Line[{}]: point[{}]: ", self.name, self.p2.name);
        println!("{}  {}  {}", self.p2.x, self.p2.y, self.p2.z);
    }

    pub fn out2(&self) {
        print!("Line[{}]: ", self.name);
        self.p1.out();
        print!("Line[{}]: ", self.name);
        self.p2.out();
    }
}

//pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
pub fn run() -> Result<(), Box<dyn Error>> {    

    let about = md_about::StAbout::new(
    	"Leonid", 
		"Nikolaevich", 
		"Kornilov",
		"Kornilov LN (Starmark)", 
		"https://github.com/KornilovLN/rust-demo.git",
		"ln.KornilovStar@gmail.com",
		"03.09.2023 19:27:00",
    );	
	about.out();
	about.waiter(4);
    //println!("{}",about.tostring());

    //--- Возможно тут еще какой-либо код может быть
    println!("\n\tHi! Struct!");

    let point1 = Point{
        x:56.0, 
        y:72.6, 
        z:99.45, name: "P1".to_string()};

    let point2 = Point{
        x:6.3, 
        y:2.6, 
        z:77.5, name: "P2".to_string()}; 

    println!("\n\tВывод точек");  
    point1.out();
    point2.out();
     
    let line: Line = Line{ p1: &point1, p2: &point2, name: "L1".to_string()};          
    let line2: Line = Line{ p1: &point1, p2: &point2, name: "L2".to_string() };  
    println!("\n\tВывод линий: метод out()");
    line.out(); 
    line2.out();   

    println!("\n\tВывод линий: метод out2()"); 
    line.out2();
    line2.out2();

    println!("\n\tСнова вывод точек посредством debug"); 
    println!("\n'debug printing' {:#?}", point1);
    println!("\n'debug printing' {:#?}", point2);

	Ok(())	
}