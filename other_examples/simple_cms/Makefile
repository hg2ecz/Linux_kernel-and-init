PROGNAME=init
DOCKER_IMGNAME=kernel-initonly

BOOTPARAM=-nographic -append "console=ttyS0 rdinit=/sbin/init ip=dhcp" -initrd initrd.img -smp 2
BOOTPARAM+=-device virtio-net-pci,netdev=net0

USERNET=-netdev user,id=net0,hostfwd=tcp::7878-:7878,hostfwd=tcp::8080-:8080,
BRIDGE=-netdev bridge,id=net0,br=br0


all:
	#rustup target add x86_64-unknown-linux-musl
	#cargo build --release --target x86_64-unknown-linux-musl
	cargo build --release

	mkdir -p initrd/lib initrd/lib64 initrd/sbin initrd/proc initrd/run
	##if [ ! -e initrd/lib64 ]; then ln -s lib initrd/lib64; fi

	cp -p target/release/${PROGNAME} initrd/sbin/init
	ldd initrd/sbin/init  # debug

	#cp -p /lib/linux-vdso.so.1 initrd/lib/ -- from kernel
	cp -p /lib/x86_64-linux-gnu/libssl.so.3 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libcrypto.so.3 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libz.so.1 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libgcc_s.so.1 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libm.so.6 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libc.so.6 initrd/lib/
	cp -p /lib64/ld-linux-x86-64.so.2 initrd/lib64/       # lib64 !!
	cp -p /lib/x86_64-linux-gnu/libzstd.so.1 initrd/lib/

	cp -r webdata initrd/
	cd initrd && find . | cpio -H newc -o | zstd > ../initrd.img && cd ..

docker-build:
	make all
	docker build -t $(DOCKER_IMGNAME) .

docker-run:
	docker run --cpus=2 -m 128M -p 7878:7878 -p 8080:8080 -i $(DOCKER_IMGNAME)

run:
	qemu-system-x86_64 -kernel ../../common/kernel/bzImage $(BOOTPARAM) $(USERNET) -m 128

run-bridge:
	sudo qemu-system-x86_64 -kernel ../../common/kernel/bzImage $(BOOTPARAM) $(BRIDGE) -m 64

clean:
	rm -rf lib lib64 initrd initrd.img init
	docker ps -a -q --filter ancestor=$(DOCKER_IMGNAME) | xargs -r docker stop
	docker ps -a -q --filter ancestor=$(DOCKER_IMGNAME) | xargs -r docker rm
	docker image rm $(DOCKER_IMGNAME)

fullclean:
	clean
	cargo clean
