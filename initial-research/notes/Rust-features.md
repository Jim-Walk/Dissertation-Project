# Key features of Rust

Rust attempts to give you speed, without giving you the power to shoot yourself in the foot. The language is designed to be ergonomic, in that care is taken to ensure code is concise, and easy to understand.

### C code
```
void f(Foo\* ptr){
    if (!ptr){
        return;
    }
    ptr->g();
}
```
### Rust code
```
fn f(ptr: Option<&Foo>){
    match ptr{
        some(ptr) => ptr.g();
        None => {}
    }
}
```
The rust code's compulsory pattern matching makes it safe. The C safety check is optional, in Rust it is not. Matching is not the same as as switch statement, it allows you to unwarp composite data types, and can be used as part of an expression.
```
let foo = match x{
    Foo::A(n) => n,
    _ => 0,
};
```
## Option type
The option type is built into the language, it has two types: Some(T) and None. (Compiler can optimise this)
option.map(add\_four) where add\_four is a function. Can be really cool with lambda functions, i.e.
```
fn maybe_add_four(y: Option<i32>) -> Option<i32>{
    y.map(|x| x + 4)
}
```
You can convert an option type to a result type with .ok\_or(err: E) and .iter()

## Result type
Similar to the option type, two types are Ok(T), and Err(e)

```
fn h(i: 32) -> Result<i32, String>{
    match i {
        i if i => Ok(i + 10),
        _ => Err(format!("Input to h less than 0, found: {}", i)
   }
}

fn main(){
    let input: i32 = ...;
    match h(input){
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {} ", e)
    }
}
```
Consider aliasing the Result type to re-use it.

## Macros vs Functions

Macros are a form of metaprogramming. A macro can take a variable number of agruments. Macros can hide returns. They're an advanced feature of rust, and are a sort of hygenic (resistant to undefined or unexpected behavior) approach to templating.


## If Let

```
if let Ok(i) = h(){
    // do something with i
}
```
If the function h returns something that matches, do something. Compare to while let, which is if let whilst it can iterate

## ?
`let i = h()?;` or even `let i = h()?.foo()?.bar` used for options.

## iters

`array.iter().for\_each(|f| f.foo());`
An iterator is a *trait* which is similar to an interface. Most iterators are lazy.
```
vec.iter()
    .map(|x| x+1)
    .filter(|x| x>0)
    .for_each(|x| println!("{}",x);
```

##
A vec is just dynamic array in Rust. You can choose to iterate over references or values.

## Error Handling
Rust approaches error handling from the perspective of an architectual decision
