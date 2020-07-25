fn main() {
    // 1. Moving and Borrowing
    let x: i8 = 1;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    let a = String::from("hello world");
    let b = a; // ownership of a is moved to b

    println!("{}", b);
    // code below will result on error
    // println!("{}", a);
    //
    let c = &b;
    println!("{}", b);
    println!("{}", c);

    fn hello(s: String) {
        println!("{}", s);
    }

    let d = b.clone();
    println!("{}", d);
    println!("{}", b);

    hello(b);
    // b is move as s on hello parameter
    // this will result in error
    // println!("{}", b);

    // 2. Struct And Traits
    struct Person {
        name: String,
    }

    impl Person {
        pub fn new(name: String) -> Self {
            Person { name: name }
        }

        pub fn print_name(&self) {
            println!("{}", self.name);
        }
    }

    let name = String::from("danu");
    let danu: Person = Person::new(name);

    danu.print_name();

    pub trait Hello {
        fn hello(&self);
    }

    impl Hello for Person {
        fn hello(&self) {
            println!("hello {}", self.name);
        }
    }

    // notice that danu still can call hello
    // eventhough Hello traits were defined
    // after danu were created
    danu.hello();

    // 3. Option and Result
    let exist: Option<i8> = Some(8);
    let mut none: Option<i8> = None;

    // unwrap is used to force Option to return its value
    println!("{}", 10 + exist.unwrap());
    // this will result an error
    // println!("{}", none.unwrap());
    none = exist;
    println!("{}", none.unwrap());

    type Res = Result<i8, String>;
    let ok: Res = Ok(10);
    let err: Res = Err(String::from("something goes wrong"));

    // 4. Pattern Matching
    if let Ok(n) = ok {
        println!("{}", 10 + n);
    }

    if let Err(msg) = ok {
        println!("{}", msg);
    }

    if let Err(msg) = err {
        println!("{}", msg);
    }

    let mut date: [(u8, u8, u64); 3] = [(0, 0, 0); 3];
    date[0] = (14, 06, 1999);
    date[1] = (15, 06, 2000);
    date[2] = (16, 06, 2001);

    for i in 0..2 {
        if let (day, _, 2000) = date[i] {
            println!("{}", day);
        }
    }

    match date[1] {
        (14, _, _) => println!("first match"),
        (15, _, _) => println!("best match"),
        _ => println!(":<"),
    }

    enum Gender {
        Male,
        Female,
        Other(String),
    }

    let gender = Gender::Other(String::from("Attack Helicopter"));

    match gender {
        Gender::Male => println!("its a male"),
        Gender::Female => println!("its a male"),
        Gender::Other(choice) => println!("its a {}", choice),
    }
}
