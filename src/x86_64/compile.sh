#!/bin/bash
printf "%s\n\n" "Cleaning any old build output. Removing:"
rm -f -v boot boot.o multiboot_header multiboot_header.o kernel.bin long_mode_init.o long_mode_init
printf "\n%s\n" "Files removed. Compiling assembly..."
nasm -f elf64 multiboot_header.asm
nasm -f elf64 boot.asm
nasm -f elf64 long_mode_init.asm
printf "%s\n" "Linking binary..."
ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o long_mode_init.o
printf "%s\n" "Building ISO..."
mkdir -p isofiles/boot/grub
cp ./grub/grub.cfg isofiles/boot/grub/grub.cfg
cp kernel.bin isofiles/boot/kernel.bin
grub-mkrescue -o markon_os.iso isofiles
printf "%s\n" "ISO successfully built."
printf "%s\n" "Run 'qemu-system-x86_64 -cdrom markon_os.iso' to boot."
echo "Done. ✅"
