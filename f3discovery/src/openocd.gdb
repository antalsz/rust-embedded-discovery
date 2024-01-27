# Connect to gdb remote server
target remote :3333

# Load will flash the code
load

# Enable demangling asm names on disassembly
set print asm-demangle on

# Enable pretty printing
set print pretty on

# Disable style sources as the default colors can be hard to read
set style sources off

# Initialize monitoring so iprintln! macro output
# is sent from the itm port to itm.txt
# Fix for book code from https://github.com/rust-embedded/discovery/issues/548
monitor stm32f3x.tpiu disable
monitor stm32f3x.tpiu configure -protocol uart -traceclk 8000000 -output itm.txt -formatter off
monitor stm32f3x.tpiu enable

# Turn on the itm port
monitor itm port 0 on

# Set a breakpoint at main, aka entry
break main

# Set a breakpoint at DefaultHandler
break DefaultHandler

# Set a breakpoint at HardFault
break HardFault

# Continue running until we hit the main breakpoint
continue

# Step from the trampoline code in entry into main
step
