PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 rdinit=/sbin/init ip=dhcp" -initrd initrd.img -smp 2
BOOTPARAM+=-device virtio-net-pci,netdev=net0

USERNET=-netdev user,id=net0,hostfwd=tcp::7878-:7878
BRIDGE=-netdev bridge,id=net0,br=br0


all:
	cd ../common/ && make                 # compile "init"
	mkdir -p initrd/sbin initrd/proc
	cp -p ../common/target/x86_64-unknown-linux-musl/release/${PROGNAME} initrd/sbin/init
	ldd initrd/sbin/init  # debug
	cd initrd && find . | cpio -H newc -o | zstd > ../initrd.img && cd ..

run:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(USERNET) -m 64

run-bridge:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(BRIDGE) -m 64

rundeb:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(USERNET) -m 128

rundeb-bridge:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(BRIDGE) -m 128

clean:
	rm -rf initrd initrd.img
	cd ../common/ && make clean
