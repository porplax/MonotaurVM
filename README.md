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

#### PLOP
---------------------
Inherits the `0x01` value. Adds a **Integer** to plate. You would use it as such:
```
BE EF BA BE 01 06 01 00
``` 
This example adds *1* to plate. 

#### PLUMMET
---------------------
Inherits the `0x02` value. Adds a **Float** to plate. You would use it as such:
```
BE EF BA BE 01 06 02 00
``` 
This example adds *0.1* to plate. 

#### SCALE
---------------------
Inherits the `0x03` value. Adds a **String** to plate. You would use it as such:
```
BE EF BA BE 01 48 65 6C 6C 6F 20 57 6F 72 6C 64 21 03 00
``` 
This example adds *Hello World!* to plate. 

#### TAKE
---------------------
Inherits the `0x04` value. Takes a value from plate You would use it as such:
```
BE EF BA BE 01 06 01 04 00
``` 
This example adds *1* to plate, then removes it.

#### DEVOUR
---------------------
Inherits the `0x05` value. Erases the entire plate. You would use it as such:
```
BE EF BA BE 01 06 01 06 01 03 00
``` 
This example adds *1* to plate, then another *1* before erasing both. 

#### INC
---------------------
Inherits the `0x06` value. Adds one to **Integer** and **Float** before is it added to plate. You would use it as such:
```
BE EF BA BE 01 06 06 06 06 06 00
``` 
This example just increments to *5* without adding to plate.

#### DNC
---------------------
Inherits the `0x07` value. Subtracts one from **Integer** and **Float**. You would use it as such:
```
BE EF BA BE 01 06 06 07 07
``` 
This example adds to *2* before subtracting *2*.

#### ISTORE
---------------------
Inherits the `0x08` value. Takes the top of the plate as an **Integer** ands stores it to whatever is in current increment. You would use it as such:
```
BE EF BA BE 01 06 01 06 08 00
``` 
This example adds one to plate then takes top of plate and stores it. 

#### ILOAD
---------------------
Inherits the `0x09` value. Takes whatever is in current increment and uses it to retrieve from that key. Adds key to plate. You would use it as such:
```
BE EF BA BE 01 06 01 06 08 06 09 00 
``` 
This example stores then loads to plate. 

