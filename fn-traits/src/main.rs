fn add<F>(closure: F) -> i32
where
    F: Fn(i32) -> i32,
{
    closure(1)
}

fn add_fn_mut<F>(mut closure: F) -> i32
where
    F: FnMut(i32) -> i32,
{
    closure(1)
}

fn add_fn_once<F, T>(closure: F, val: &T) -> T
where
    F: FnOnce(&T) -> T,
{
    closure(val)
}

fn main() {
    let mut number = 1;

    let result = add(|i| number + i);
    println!("{}", result);

    // compile error:
    let _result = add(|i| {
        // cannot assign to `number`, as it is a captured variable in a `Fn` closure
        // number += i;
        i
    });

    let result = add_fn_mut(|i| {
        number += i;
        number
    });

    println!("{}", result);

    let world = "world".to_string();
    {
        let mut hello = "hello".to_string();

        let closure = move |i: &String| {
            hello.push_str(i);
            hello
        };

        let result = add_fn_once(closure, &world);

        // compile error:
        // use of moved value: `closure`
        // value used here after move
        //let result2= add_fn_once(closure, &world);

        println!("{}", result);
    }
}
