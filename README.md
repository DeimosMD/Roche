## The Roche Programming Language

The **main function** is where code starts to execute.
It is written as the following.

`
fn main()! {...}
`

The **variable declaration** syntax is the following.

`
TYPE NAME = VALUE;
`

And written with a `~` if mutable, like for example:

`
i32~ my_integer = 126;
`

**functions** are written with the following syntax's and examples.  

no parameters, no return type:  
`
fn NAME()! { BODY }
`  
`
fn do_something()! {...}
`  

no parameters, yes return type:  
`
fn NAME() -> TYPE { BODY }
`  
`
fn do_another_thing() -> u16 {...}
`  

yes parameters, no return type:  
`
fn NAME(TYPE NAME, TYPE NAME, ect.)! { BODY }
`  
`
fn some_other_task(str my_string, u128 big_number, i8 smaller_number)! {...}
`  

yes parameters, yes return type:  
`
fn NAME(TYPE NAME, TYPE NAME, ect.) -> TYPE { BODY }
`  
`
fn some_function(u32 a, u32 b) -> u32 {...}
`