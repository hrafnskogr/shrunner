# shrunner
Simple Shellcode runner in Rust

Two version of a simple program that can run a shellcode, for shellcode testing purposes.
Inspired by Sylvain Kerkour (kerkour.com).


### hex2bin.ps1
Takes an hex string as first and only positional argument :

```.\hex2bin.ps1 DEADBEEF.........```

Then outputs the hex string into a file (shellcode.bin)
