Ranald's Bones
===

A simple simulator for the [Ranald's Bones](http://vermintide.gamepedia.com/Ranald%27s_Bones) mini game at the end
of a match of Vermintide.  

This program runs on both Linux and Windows. Flags in the make file  
and the actual code change depending on the operating system. The Linux version  
uses OpenSSL to generate random numbers, while the Windows version uses  
the built it rand() method.

Requirements
---
* openssl headers, these can be found in most package managers
  * [libssl-dev](http://packages.ubuntu.com/trusty/libssl-dev)
  * [openssl-devel](https://rpmfind.net/linux/rpm2html/search.php?query=openssl-devel)
* g++ version capable of using the c++11 standard

Compilation
---
if all is well all that is needed is to run 'make' in the 'src/' directory
```
bash$ make
```

Dice
---
| Dice Type         | Chance of Success |
| ------------------|:------------------|
| Tome              | 66.667%           |
| Grimoire          | 100%              |
| Cursed Bonus Dice | 50%               |
| Regular Dice      | 33.333%           |
