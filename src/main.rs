struct Employee {
    name:String,
    company:String,
    age:u32
}
fn add_one(e: &mut i32) {
    *e+= 1;
}
#[derive(Debug)]
enum GenderCategory {
    Male,Female
}
fn main() {
    println!("Hello, world!");
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i);
    let j = Box::new(i);
    println!("{}",4==i);
    println!("{}",4==*j);

    let mut emp1 = Employee {
        company:String::from("TutorialsPoint"),
        name:String::from("Mohtashim"),
        age:50
    };
    emp1.age = 40;
    println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);

    let male = GenderCategory::Male;
    let female = GenderCategory::Female;

    println!("{:?}",male);
    println!("{:?}",female);
}

