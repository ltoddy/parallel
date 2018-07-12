### Why is rust.

Above all, I need a binary and dynamic link library, so I can use other language invoke it(like: "ffi").

I choose rust not because of rust, but because of the rust package manager.

I'm not going to write Makefile.

If I touch files named "src/lib.rs" and "src/main.rs", cargo will automatically generate dynamic link library and binary.

Python and rust have similarities in the design of ABI.
