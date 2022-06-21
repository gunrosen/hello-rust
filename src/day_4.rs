
// Bài tập cho trait
// Đề bài : Implement trait Iterator (của thư viện Rust) cho kiểu dữ liệu Struct sau

//https://doc.rust-lang.org/std/iter/trait.Iterator.html

// struct Fibonacci {
//     a: u32,
//     b: u32,
// }
// Như mọi người đã biết Dãy Fibonacci có quy luật như sau
// Dãy Fibonacci là dãy số bắt đầu bằng 0 với 1. Mọi số tiếp theo
// đều là tổng của 2 số trước đó
// Ví dụ: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, ...


// Trong trường hợp bài toán này
// Khởi tạo ban đầu sẽ là
// struct Fibonacci {
//     a = 1,
//     b = 0,
// }

// Một số kiến thức để làm dc bài này: Trait, Generic Type, Associated type,
// Gợi ý có sườn như sau:

#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let temp:u32 = self.a + self.b;
        self.b = self.a;
        self.a = temp;
        Option::from(temp)
    }
}

fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}

pub(crate) fn test_day_4_ex_1() {
    for number in fibonacci_numbers() {
        println!("{}", number);
    }
}


// Kết quả :
// Nó sẽ iter mãi và hiện kết quả như sau:
// 1
// 1
// 2
// 3
// 5
// 8
// 13
// 21
// 34
// 55
// 89
// 144
// 233
// 377
// ...



// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime

use std::fmt;
struct StrDisplayable <'a>(Vec<&'a str>);

impl <'a>fmt::Display for StrDisplayable<'a> {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

pub(crate) fn test_day_4_ex_2() {
        let vec: Vec<&str> = vec!["a","bc","def"];
        let vec_Foo = StrDisplayable(vec);
        println!("{}",vec_Foo);
}