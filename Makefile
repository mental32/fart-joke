arch   ?= x86_64

export common := $(abspath ./common)
export target ?= target-$(arch)
export kernel_blob := $(abspath ./build/kernel-$(arch).bin)

GRUB_MKRESCUE = grub-mkrescue
QEMU := qemu-system-$(arch)

iso    := ./build/kernel-$(arch).iso
grub_cfg := $(common)/grub.cfg

QEMU_MEM := 8M
QEMU_SMP := 4
QEMU_ARGS := -m $(QEMU_MEM) -smp $(QEMU_SMP) -nographic -net nic,model=e1000 -machine q35

.PHONY: all kernel

all: $(iso)

qemu: all
	$(QEMU) -drive format=raw,file=$(iso) $(QEMU_ARGS)

$(iso): kernel $(grub_cfg)
	mkdir -p ./build/isofiles/boot/grub
	cp $(kernel_blob) build/isofiles/boot/kernel.bin
	strip build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	$(GRUB_MKRESCUE) -o $(iso) build/isofiles
	rm -r build/isofiles

kernel:
	mkdir -p build
	make -C ./kernel
