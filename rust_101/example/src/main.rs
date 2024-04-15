#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut rect1 = Rectangle {         // If you want to change the values of this instanse, it needs to be mutable
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    
    rect1.width = 40;
    println!("rect1 is {:?}", rect1);
    
    change(&mut rect1);                      // Pass a mutable reference to `change`
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)                         // FIX: area(&rect1)
    );
    
    /* Thie would not work, becuse area is taking ownership over rect1.
    rect1.width = 30;
    println!("rect1 is {:?}", rect1);
    */
}

fn change(rectangle: &mut Rectangle)        // If you pass the 'rectangle' by value, it means that it gets its own copy of Rectangle that is separate
{                                           // from rect1 in main. Modifications inside change won't affect rect1.
    rectangle.width = 60;
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}