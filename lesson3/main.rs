
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
}

// 计算数组中除了当前元素其他元素的乘积
fn array_product(a: &[i32], b: &mut Vec<i32>) {
    let mut temp = 1;

    for i in 0..a.len() {
        b[i] = temp;
        temp = temp * a[i]; // 不考虑乘法溢出
    }
    temp = 1;
    for i in (0..a.len()).rev() {
        b[i] = b[i] * temp;
        temp = temp * a[i];
    }
}

// 1+max(1,1+4+2)=8
#[derive(Debug)]
enum MyEnum {
    A(u8),
    B,
    C { x: u8, y: i32, c: u16 },
}
//1+max(1,4+4)=9  => 对齐到12 字节
enum MyEnumAlign {
    A(u8),
    B,
    C { x: i32, y: i32 },
}

#[derive(Debug)]
enum MyEnum_1 {
    A,
}
#[derive(Debug)]
enum MyEnum_2 {
    A(u8, u8),
    B,
}

#[derive(Debug)]
enum MyEnum_3 {
    A {},
}
#[derive(Debug)]
enum MyEnumA {
    A = 255,
}

#[derive(Debug)]
enum MyEnumB {
    A,
    C,
    B { x: u16, y: u16 },
}



struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // 创建一个新的空栈
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // 将元素压入栈顶
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // 从栈顶移除元素并返回
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // 查看栈顶元素但不移除
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}
// 计算字符串中每个单词出现的次数
fn word_count(s: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for word in s.split_whitespace() {
        let count = map.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }
    map
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    stock: u32,
}

struct Library {
    books: HashMap<String, Book>,
}

impl Library {
    // 创建一个新的图书馆
    fn new() -> Self {
        Library {
            books: HashMap::new(),
        }
    }

    // 添加书籍
    fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn.clone(), book);
    }

    // 查询库存
    fn get_stock(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }

    // 更新库存
    fn update_stock(&mut self, isbn: &str, new_stock: u32) -> bool {
        if let Some(book) = self.books.get_mut(isbn) {
            book.stock = new_stock;
            true
        } else {
            false
        }
    }

    // 删除书籍
    fn remove_book(&mut self, isbn: &str) -> bool {
        self.books.remove(isbn).is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::array_product;
    use std::mem::size_of;
    use crate::MyEnum;
    use crate::MyEnumAlign;
    use crate::MyEnum_1;
    use crate::MyEnum_2;
    use crate::MyEnum_3;
    use crate::MyEnumA;
    use crate::MyEnumB;


    #[test]
    fn test_array_product() {
        let arr = [2, 3, 4, 5];
        let mut b: Vec<i32> = vec![1; arr.len()];
        array_product(&arr, &mut b);
        for i in b.iter() {
            println!("products is {}", i);
        }
    }

    #[test]
    // 计算枚举类型的大小
    fn enum_mem() {
        assert!(size_of::<MyEnum>() == 8);
        assert!(size_of::<MyEnumAlign>() == 12); // 对齐到12 字节
        assert!(size_of::<MyEnum_1>() == 0);
        assert!(size_of::<MyEnum_2>() == 3);
        assert!(size_of::<MyEnum_3>() == 0);
        assert!(size_of::<MyEnumA>() == 0);
        assert!(size_of::<MyEnumB>() == 6); //对齐到6 字节
        println!("size of MyEnum is {}", size_of::<MyEnum>());
        println!("size of MyEnum2 is {}", size_of::<MyEnumAlign>());
        println!("size of MyEnum_1 is {}", size_of::<MyEnum_1>());
        println!("size of MyEnum_2 is {}", size_of::<MyEnum_2>());
        println!("size of MyEnum_3 is {}", size_of::<MyEnum_3>());
        println!("size of MyEnumA is {}", size_of::<MyEnumA>());
        println!("size of MyEnumB is {}", size_of::<MyEnumB>());
    }

    // 测试stack
    #[test]
    fn test_vec() {
        let mut stack = crate::Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        println!("Top element is: {:?}", stack.peek());

        while let Some(top) = stack.pop() {
            println!("Popped: {}", top);
        }
        // 输出：
        // Popped: 3
        // Popped: 2
        // Popped: 1
    }

    // 测试单词计数
    #[test]
    fn test_word_count() {
        let s = "Hello world! Hello Rust!";
        let word_count = crate::word_count(s);
        println!("{:?}", word_count);
    }

    // 测试图书馆
    #[test]
    fn test_library() {
        let mut library = crate::Library::new();
        let book1 = crate::Book {
            title: "Rust Programming".to_string(),
            author: "John Doe".to_string(),
            isbn: "1234567890".to_string(),
            stock: 10,
        };
        let book2 = crate::Book {
            title: "Rust Programming 2".to_string(),
            author: "John Doe".to_string(),
            isbn: "0987654321".to_string(),
            stock: 5,
        };
        library.add_book(book1);
        library.add_book(book2);
        println!("{:?}", library.get_stock("1234567890"));
        println!("{:?}", library.get_stock("0987654321"));
        println!("{:?}", library.update_stock("1234567890", 20));
        println!("{:?}", library.remove_book("0987654321"));
        println!("{:?}", library.get_stock("1234567890"));
    }
}

