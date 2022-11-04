# Monotaur
### `Introduction`
**Monotaur** implements simple math operations, storage operations, stack operations, system calls, import operations. The VM even has ways to idenitfy proper `.cw` files, which is it's input files. It's a simple project I made to better understand how programming languages work, how CPUs work, and how VMs work. The stack in Monotaur is called the plate. The plate has a size of `128`. 

### `Instructions`
#### HALT
--------------------- 
Inherits the `0x00` value. Halts the current program it is executing. Here's its context.
```
BE EF BA BE 01 00
```
The program above just validates the program and exits. 

---------------------
#### PLOP
---------------------
Inherits the `0x01` value. Adds a **Integer** to plate. You would use it as such:
```
BE EF BA BE 01 06 01 00
``` 
This example adds *1* to plate. 
