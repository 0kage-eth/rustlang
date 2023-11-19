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

- 
