struct Point { x: u32, y: u32 }
enum Option<T> { None, Some(T) }

fn map<S, T, F: Fn(S) -> T>(opt: Option<S>, f: F) -> Option<T> {
    match opt {
        Option::None    => Option::None,
        Option::Some(s) => Option::Some(f(s)),
    }
}
