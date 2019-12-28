#!/bin/bash
printf "%s\n\n" "Cleaning any old build output. Removing:"
rm -f -v boot boot.o multiboot_header multiboot_header.o kernel.bin
printf "\n%s\n" "Files removed. Compiling assembly..."
nasm -f elf64 multiboot_header.asm
nasm -f elf64 boot.asm
printf "%s\n" "Linking binary..."
x86_64-elf-ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o
echo "Done. ✅"
