# obfuscate-rs

## TODO:  
### short-term  
[x] move this and obfuscate into one workspace, where obfuscate-str is the exported crate and obfuscate-str-proc is the proc_macros  
[] bubble up encryption errors to the user  
[] write a well-defined README that explains that this isn't proper encryption and that it should be only used to make it harder to understand hard-coded strings in the binary  
[] also add that there is no malicious intent and this is to offer people exercises in ingenuity and creativity for situations such as Capture The Flags (reverse engineering) and to better understand rust-written malware since it's becoming popular  
[] add basic key encryption using XORs  
[] offer a non-embeded key option, in order to generate a key and print to user so that they can include it as an environment variable instead (this will have to be statically defined flags)  
[] check licenses and add one. Read about the differences between Apache 2.0 and MIT  
  
### mid-term  
- add benchmarking to evaluate overhead  
- use these libraries to create a cargo-obfuscate that does this automatically for all strs in the program  
  
### long-term  
- non-std situations (find relevant crate)  
- implement async version for cargo-obfuscate (to await on the calculation of the first time every str)  
