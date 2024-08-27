PROGNAME=init
DOCKER_IMGNAME=kernel-initonly

BOOTPARAM=-nographic -append "console=ttyS0 rdinit=/sbin/init ip=dhcp" -initrd initrd.img -smp 2

# ip link add link enp3s0 name macvtap0 type macvtap mode bridge; ip link set macvtap0 up
#ETH=-netdev tap,id=tap0,ifname=tap0,script=no,downscript=no,vhost=on -device virtio-net-pci,netdev=tap0,mac=52:54:00:12:34:56

# For this *_eth version you need to create a br0 bridge width a dhcp server.
ETH=-netdev bridge,id=net0,br=br0 -device virtio-net-pci,netdev=net0


all:
	#rustup target add x86_64-unknown-linux-musl
	#cargo build --release --target x86_64-unknown-linux-musl
	cargo build --release

	mkdir -p initrd/lib initrd/lib64 initrd/sbin initrd/proc initrd/run

	# see ldd target/release/init
	#cp -p /lib/linux-vdso.so.1 initrd/lib/ -- from kernel
	cp -p /lib/x86_64-linux-gnu/libssl.so.3 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libcrypto.so.3 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libz.so.1 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libgcc_s.so.1 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libm.so.6 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libc.so.6 initrd/lib/
	cp -p /lib64/ld-linux-x86-64.so.2 initrd/lib64/

	cp -p target/release/${PROGNAME} initrd/sbin/init
	cp -r webdata initrd/
	cd initrd && find . | cpio -H newc -o | zstd > ../initrd.img && cd ..

docker-build:
	make all
	docker build -t $(DOCKER_IMGNAME) .

docker-run:
	 docker run -d -p 7878:7878 -p 8080:8080 $(DOCKER_IMGNAME) -i

run_noeth:
	qemu-system-x86_64 -kernel ../../common/kernel/bzImage $(BOOTPARAM) -m 64

run:
	sudo qemu-system-x86_64 -kernel ../../common/kernel/bzImage $(BOOTPARAM) $(ETH) -m 64

clean:
	rm -rf lib lib64 initrd initrd.img init
	docker ps -a -q --filter ancestor=$(DOCKER_IMGNAME) | xargs -r docker stop
	docker ps -a -q --filter ancestor=$(DOCKER_IMGNAME) | xargs -r docker rm
	docker image rm $(DOCKER_IMGNAME)

fullclean:
	clean
	cargo clean
