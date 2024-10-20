# rs-lc3-vm

This is my attempt to develop *kind of* Low-Level things using Rust. In particular, I wanted to build my own implementation of [LC3](https://en.wikipedia.org/wiki/Little_Computer_3) using [this article about creating a VM in C](https://www.jmeiners.com/lc3-vm/) as a guideline. 

The result is a working prototype with behavior similar to [this](https://wchargin.com/lc3web/) emulator. You can play around with it and try to optimize things, because I really didn't think that much about the code optimization and *rusty* things such as closures and iterators. My goal was to try and make this work.

**And it's working!** 

However, there are some issues. Head over to the corresponding section to see them.