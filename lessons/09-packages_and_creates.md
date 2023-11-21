## Packages Modules And Crates

- As project grows, we need to split into multiple modules
- package can contain multiple binary crates and optionally one library crate
- code can be encapsulated to reuse at a higher level -> concept of interfaces
- way you write determines which parts of code are public and which parts are private
- **packages** - features that helps us build, test and share crates
- **crates** - tree of modules that produce library or executable
- **modules** - lets us control org scope and privacy of paths
- **struct** - way of naming an item 


### Packages and Crates

- Crate is smallest amount of code that rust compiler considers at a time
- if we run `rustc` instead of cargo and pass single file, compiler considers that file as a crate
- crate can be  - binary or library crate
- binary crates are programs you can compile to executable
- each binary crate has a function called `main()` that defines what happens when executable runs
- library crates don't have `main` function, they don't compile into an executable
- they define functionality shared in multiple projects (eg. the rand crate). When we say crate most of the time, we are referring to the library crate

- crate root is source file that Rust compiler starts from and makes up root module of crate
- package is a bundle of one or more crates that provides specific functionality

- package contains Cargo.toml that describes how to build those crates
- Cargo itself is a package that contains the binary crate for command line tool 
- Cargo package also contains a library crate that binary crate depends on

- package can contain as many binary crates but at most, only one library crate
- Cargo follows a convention that src/main.rs is the crate root of binary crate with same name as the package
- likewise, if cargo has src/lib.rs, package contains a library crate with same name as package
and src/lib.rs is the crate root

- if project has src/main.rs and src/lib.rs, it has 2 crates: a binary and library, both with same name as package



### Modules

- A module can be declared with the `mod` keyword
- code in a module is priavte by default from its parent modules -> to make a module public, declare it as `pub mod`
- `use` keyword -> creates shortcuts to items to avoid long paths

for eg we can say `use crate::garden::veggies::brinjal` and then use `brinjal` directly in that scope


- contents of main or lib files form a module names `crate` at the root of crate's module structure, known as module tree

- if module A is contained in module B, we say B is parent of A or A is child of B



### Paths

- to show rust where to find the path of a given item in module tree, we use path similar to file navigation system

- path - can be absolute or relative. absolute path starts with literal `crate`
- relative path starts with current module and uses `self` `super` keywords
- both absolute and relative are followed by identifiers separated by `::`

- items in parent module can't use private items in the child modules but items in child module can see the context in which they are defined

- We can expose modules using the `pub` keyword

- Making module public does not audtomatically give us access to the inner contents of the module, eg. functions. by default, functions defined in a module are private -> to allow access from other modules, we need to make them public as well

- We can start relative paths that begin in parent module rather than current module by using `super` keyword

```
mod parernt_module{
   fn parent_func(){}

   mod child_module{
     fn child_func1(){}

     fn child_func2(){
       child_func1(); // same context
       super.parent_func(); // go to the context of parent module
     }    
   }
}

```

- Using super makes sense when you are confident that the relationship between parent and child will remain fixed



### Enums and Struct as Public
- Structs and enums can also be made public by the `pub` keyword. Although the struct is public and accessible, each of its elements are still private -> to make them public, we have to use `pub` keyword on each of them separately

- In contrast, if we make enum public, all its variants are also public. default behavior here for enum is to be public, if the enum is declared with `pub` keyword



### Use keyword

- Defining large paths inside the code is cumbersome. That is where `use` comes into play. `use` creates a shortcut for path in scope and makes calling external functions or structs easy (with smaller signature)

- Each use has a specific scope - and shortform notations are only applicable in that scope.


For eg.


```
   mod parent{
      
     pub fn parent_fnc1(){}
     
     
   }
  
  mod other_parent{
     

      use crate::parent::parent_fnc1;
      fn other_parent_fnc(){
           parent_fnc1(); // in this case, since use is in scope, this is valid
      }

     mod child{

       fn child_func(){
          // parent_fnc1();  ---- This will not work as scope has changed
          super.parent_fnc1(); // this will work since the use has the same scope as super
      }
     } 
      
  }

```


- We can specify alias for a function or module using the `use`


```
   use std::fmt::Result;
   use std::io::Result as ioResult; //-n not to be confused with above, we use a different name

   let func1() -> Result{}

   let func2() -> ioResult<()>{}


```

- Reexporting names with `pub use` -> we can bring a name into scope using `use` keyword and also make that public using `pub` keyword - so we combine pub and use to bring an item into scope and also making that available to others to bring into their scope - this technique is called re-exporting

- re-exporting is done when the internal code structure differs from conventional way in which programmers call code or think about code structure. To make both consistent, re-exporting can be used




### Using External Packages


- We can add external packages to `Cargo.toml` and use them inside our project by bringing them into scope with `use` keyword

- In the first chapter we added

`rand=0.8.5` in our Cargo.toml

- And then we added `use rand::Rng` to bring the package into use

- Members of rust community have created many packages that can be used by bring them into scope and adding them to Cargo toml


- Also, packages like `std` are shipped in default Rust. But they still need to be brought into scope by `use`
for eg. `use std::collections::HashMap;`


- We can also use nested paths to use multiple items from the same crate or module


```
use std::cmp::Ordering;
use std::io;

```


can be changed to 

```
use std::{cmp::Ordering, io};
```
 

In cases where there multiple items from same crate/module are picked, we can use the `self` keyword


```

use std::io;
use std::io::Write;
```

can be modified to

```

use std::io::{self, Write};
```

Above line brings `std::io` and `std::io::Write` into scope




- If we want to bring all public items into scope, we can use the `*` as follows

```
use std::collections::*;
```  

- all public items, structs/sub modules/enums will be brought into scope. We call this the **Glob Operator**

- Glob operator is often used to bring everything under scope while testing - we will see in future...



### Moving modules into files


- All mmodules at crate root level can be defined in their own files (instead of lib.rs) with the file name same as module name

- All sub modules for module with crate root scope can be defined in a folder with name as module

- Module can then include the sub modules in its scope using the `use` keyword.

This placement is similar to file directory

 



