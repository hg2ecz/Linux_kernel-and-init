IMGFILE=cdimage.iso
PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 root=/dev/vda ro ip=dhcp" -drive file=$(IMGFILE),format=raw,if=virtio -smp 2
BOOTPARAM+=-device virtio-net-pci,netdev=net0

USERNET=-netdev user,id=net0,hostfwd=tcp::7878-:7878
BRIDGE=-netdev bridge,id=net0,br=br0


all:
	cd ../common/ && make                  # compile "init"
	mkdir -p cdimage/sbin cdimage/proc
	cp -p ../common/target/x86_64-unknown-linux-musl/release/${PROGNAME} cdimage/sbin/init
	ldd cdimage/sbin/init  # debug
	mkisofs -input-charset utf-8 -o $(IMGFILE) -V "Test CD" -R cdimage/

run:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(USERNET) -m 64

run-bridge:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(BRIDGE) -m 64

rundeb:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(USERNET) -m 128

rundeb-bridge:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(BRIDGE) -m 128

clean:
	rm -rf cdimage $(IMGFILE)
	cd ../common/ && cargo clean
