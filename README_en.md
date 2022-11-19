# KittenVM
Roughly an imaginative LVM, I guess.

## Use

### Warning:About stability!
The most recommended version at the moment (during testing) is the latest alpha version! For example ``1.0.0-1``  
Stable versions will not have a crossbar version, such as ``1.0.0``

### Rust
```
[dependencies]
KittenVM = "1.0.0"
```

```
use KittenVM::API;

let a = API::stream(new a);
//----------OR--------------
let result = API::file("file.kvm");
//----------OR--------------
// This method is recommended!
let run = let mut run: KittenVM = vm::machine::KittenVM::default();
let a = run.dynamic_memory.new_mem(vm::value_type::Type::int(36));
```
### C/C++
Make sure KittenVM.lib is in the same directory as libKittenVM.d; if using a build tool like CMake, make sure it's referenced correctly!
[Staticlib download](https://gitee.com/Shirasawa-CN/kitten-vm/releases)

## Features
This toy has many strange and brainy features, please move to [README](. /docs/README.md)

## Use (command)
## Create space
```
new name
```
Create a memory space named name

## Move data
```
mov name,4
```
Move the number 4 to the name space

```
mov name1,name2
```
Move the value of name2 to name1, then name2 becomes None

## Memory recovery
```
add_gc a
```
Add a to the recycle list
```
free
```
Clear the memory in the recycle list

## Operations
add and div mul or sll sra sud xor The format of each of these instructions is as follows

```
expr rs1,rs2,target
```

## Use (function)
## Create space
```
let name = run.dynamic_memory.new_mem(vm::value_type::Type::int(36));
```
Create a memory space named name

## Move data
```
run.mov(4,name);
```
Move the number 4 to the name space

```
run.mov(name1,name2);
```
Move the value of name2 to name1, at which point name2 becomes None

## memory recovery
```
run.add_gc(a);
```
Add a to the recycle list
```
run.free();
```
Clean up memory in the recycle list

## Operations
add and div mul or sll sra sud xor The format of each of these instructions is as follows

```
expr(rs1,rs2,target);
```

Translated with www.DeepL.com/Translator (free version)