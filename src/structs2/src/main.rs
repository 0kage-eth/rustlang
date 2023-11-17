fn main() {

    // 1. call basic implementation
    let l = 30;
    let b = 20;

    println!("1. area basic: {}", area(l,b));
    
    let r_tuple = (l, b);

    println!("2. area tuple: {}", area_tuples(r_tuple));

    let r = Rectangle{length: l, breadth: b};

    println!("3. area struct: {}", area_struct(&r));
    
    println!("printing rectangle: {:?}", r);    
    println!("printing rectangle better formatting: {:#?}", r);
    
    let s = dbg!(20 * l); 
    println!("value of s is {}", s);

    dbg!(&r); //note we passed a reference so that ownership does not get transferred

    println!("area by defining method inside struct {}", r.area());

    println!("breadth the variable, {}", r.breadth);
    println!("breadth the function, {}", r.breadth());

    let r2 = Rectangle{length: 10, breadth: 4};
    let r3 = Rectangle{length: 4, breadth: 2};

     println!("r2 contained in r? {}", r.contains(&r2));
     println!("r3 contained in r? {}", r.contains(&r3));

     let sq = Rectangle::square(10);

     println!("square {:#?}",sq); 
}

#[derive(Debug)]
struct Rectangle{
    length: u32,
    breadth: u32,
}

impl Rectangle {
   fn area (&self) -> u32 {
     self.length * self.breadth
  }

  fn breadth(&self) -> bool{
    self.breadth > 0
  }

  fn contains(&self, other: &Rectangle) -> bool{
     other.length <= self.length && other.breadth <= self.breadth	
  }
}

impl Rectangle{
    fn square(side: u32) -> Self{
      Self{length: side, breadth: side}
    }

}

// crude implementation
fn area(length: u32, breadth: u32) -> u32 {
   length * breadth
}
// this is crude implementation
// but we don't know that length and breadth are part of one group
// each time we need to handle two isoplated parameters



// implementation using tuples
fn area_tuples(dimensions: (u32, u32))->u32 {
    dimensions.0 * dimensions.1 // problem here is that this is not intuitive
}
// have no idea what dimensions.0 and dimensions.1 mean


// using a struct as a immutable reference
fn area_struct(rect: &Rectangle) -> u32{
  rect.length * rect.breadth
} 

// this is much cleaner but still, this function is separate from the Rectangle struct
// even better way for representing this


