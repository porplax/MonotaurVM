# *Instructions*
```
HALT [0x00] | Stops the Monotaur program completely. 

PLOP [0x01] | Adds an integer to plate.
PLUMMET [0x02] | Adds a float to plate.
SCALE [0x03] | Adds a string to plate.

TAKE [0x04] | Take one from plate.
DEVOUR [0x05] | Clear entire plate.

INC [0x06] | Increments one to integer, float.
DNC [0x07] | Decrements one to integer, float.

ISTORE [0x08] | Stores integer to fridge. 
ILOAD [0x09] | Loads a value from fridge onto plate.
FSTORE [0x0A] | Stores float to fridge.
FLOAD [0x0B] | Loads a value from fridge onto plate.

IMPORT [0x16] | Imports a chow file. 
```

ILOAD and FLOAD is marked for removal in the future. 

# *Context*
In order to work with integers, you use the `INC` instruction in order to achieve desired value.
So if you wanted to get 5 and add it to plate, you have to do this:
```
BE EF BA BE 01 06 06 06 06 06 01 00
```
For every `06` is a `INC` function. 

# *What makes a .chow file*
Every chow file needs to start with `0xBEEFBABE` and the following chow version number. 
```
BE EF BA BE 01 00 
```
Chow files also need to end with `00`. Which will halt the program.
