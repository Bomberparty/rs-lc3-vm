# rs-lc3-vm

This is my attempt to develop *kind of* Low-Level things using Rust. In particular, I wanted to build my own implementation of [LC3](https://en.wikipedia.org/wiki/Little_Computer_3) using [this article about creating a VM in C](https://www.jmeiners.com/lc3-vm/) as a guideline. 

That kind of works, but there are a lot of unsafe coversions which rust kind of warns me about, but doesn't care at all, and that's why I've spent a whole day trying to make this, but didn't succeed. 

If you know how to solve this, please head over to the Issues and tell about the problems, other than buffer overflows and obviously, run with backtrace, so I can see what's wrong.

Wish you the best, from brpxd