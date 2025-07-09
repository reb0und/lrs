enum Status {
    Value(u32),
    Stop,
}

fn main() {
    println!("{}", do_twice(add_one, 5));

    let list_of_nums = vec![1, 2, 3];
    let list_of_strs: Vec<String> = list_of_nums.iter().map(|i| i.to_string()).collect();

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
