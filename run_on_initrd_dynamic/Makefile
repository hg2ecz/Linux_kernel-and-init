PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 rdinit=/sbin/init ip=dhcp" -initrd initrd.img -smp 2
BOOTPARAM+=-device virtio-net-pci,netdev=net0

USERNET=-netdev user,id=net0,hostfwd=tcp::7878-:7878
BRIDGE=-netdev bridge,id=net0,br=br0

DOCKER_IMGNAME=kernel-initonly

all:
	cd ../common/ && cargo build --release
	mkdir -p initrd/lib initrd/lib64 initrd/sbin initrd/proc

	cp -p /lib/x86_64-linux-gnu/libgcc_s.so.1 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libc.so.6 initrd/lib/
	cp -p /lib64/ld-linux-x86-64.so.2 initrd/lib64/

	cp -p ../common/target/release/${PROGNAME} initrd/sbin/init
	cd initrd && find . | cpio -H newc -o | zstd > ../initrd.img && cd ..

run:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(USERNET) -m 64

run-bridge:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(BRIDGE) -m 64

rundeb:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(USERNET) -m 128

rundeb-bridge:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(BRIDGE) -m 128


docker-build: all
	docker build -t $(DOCKER_IMGNAME) .

docker-run:
	 docker run -d -p 7878:7878 $(DOCKER_IMGNAME)

clean:
	rm -rf initrd initrd.img
	cd ../common/ && cargo clean
	docker ps -a -q --filter ancestor=$(DOCKER_IMGNAME) | xargs -r docker stop
	docker ps -a -q --filter ancestor=$(DOCKER_IMGNAME) | xargs -r docker rm
	docker image rm $(DOCKER_IMGNAME)
