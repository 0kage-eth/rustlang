fn main() {
	let numbers = vec![23,28,32,12,92, 16];
	let mut largest = &numbers[0];

	for number in &numbers{
		if largest < number{
			largest = number;
		}	
	}
	
	println!("largest number is {largest}");

	let numbers2 = vec![22,12,77,56,42];
	let result = largestFn(&numbers2);
	println!("largest number is {result}");

	let mut p = Point{x: 5, y: 10 };

	println!("x value of p {}", p.x());

	let p2 = Point2{x: 5, y:2.4};

	let p3 = Point2{x: 1.2, y: false};

	let p4 = p2.mix(p3);
	println!("value of mixed p {}, {}", p4.x, p4.y);
}

fn largestFn(numbers: &[u32]) -> &u32 {
	
	let mut result = &numbers[0];

       for number in numbers {
		if number > result {
			result = number;
		}	
	}
	result
}


fn largestFnGeneric<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
	let mut result = &list[0];

	for value in list{
		if result < value { // @note that this operator has to be defined for type
			result = value;
		}
	}
	result
} 

// @note in the above function, we are restrict type T to only those types
// that implement the std::cmp::PartialOrd
// if there is no such implementation, result < value does not make sense


// using generic types in method definitions


struct Point<T>{
    x: T,
    y: T,	
}

impl<T> Point<T>{
	fn x(&self) -> &T{
		&self.x
	}
}



struct Point2<T,U>{
    x: T,
    y: U,
}


impl<X1,Y1> Point2<X1,Y1>{

    fn mix<X2,Y2>(self, p: Point2<X2,Y2>) -> Point2<X1,Y2>{
	Point2{x:self.x,y: p.y}
    }

}






