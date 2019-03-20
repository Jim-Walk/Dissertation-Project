fn foo(input: Option<i32>) -> Option<i32>{
    if input.is_none(){
        return None;
    }

    let input = input.unwrap();
    if input < 0 {
        return None;
    }
    Some(input)
}

fn bar(input: Option<i32>) -> Result<i32, String>{
    match foo(input){
        Some(n) => Ok(n),
        None => Err("woops".to_string()),
    }
}

fn foo2(input: Option<i32>) -> Option<i32>{
    input.and_then(|i| {
        if i < 0 {
            None
        } else {
            Some(i)
        }
    })
}

// does the job of foo
fn foo3(input: Option<i32>) -> Option<i32>{
    input.filter(|i| i>= &0)
}

fn bar2(input: Option<i32>) -> Result<i32, String>{
    foo(input).ok_or("whoops".to_string())
}

fn add_four(foo: i32) -> i32{
    foo + 4
}

fn main() {

    /* chain chains two iterators together
    let vec = vec![0,1,2,3];
    for (i,v) in vec.iter()
                    .chain(Some(42).iter())
                    .enumerate(){
        println!("{}: {}", i,v); 
   }
   */
   let bar = Some(3);
   bar.map(add_four);
}
