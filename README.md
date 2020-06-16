# dnsServer
let's play around with DNS stuff on Rust! Wooo! Optimization!

So---

this is basically me (Red) playing around and learning via copying (WHICH IS A TOTALLY VALID WAY TO LEARN for just beginning) a DNS 
server with Rust. 

What I've learned so far

--Rust can be implemented to be very meticulous with handling bytes. We can see this in the buffer manipulation
and read methods here

--DNS packets are relatively optomized and limited to 512 byes. That's cool! There's a lot that goes in those 512 bytes.
  so far I've been doing things like reading the length header for domain name labels and identifying where a jump to an external label occurs.

--Error handling and finish stasuses are really nicely handled with the Result varients 

--Github desktop has been pretty convient in commiting changes and going through the cargo build process. Thanks GitHub!

