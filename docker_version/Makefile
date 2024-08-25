IMGNAME=kernel-initonly
PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 rdinit=/sbin/init ip=dhcp" -initrd initrd.img -smp 2

# ip link add link enp3s0 name macvtap0 type macvtap mode bridge; ip link set macvtap0 up
#ETH=-netdev tap,id=tap0,ifname=tap0,script=no,downscript=no,vhost=on -device virtio-net-pci,netdev=tap0,mac=52:54:00:12:34:56

# For this *_eth version you need to create a br0 bridge width a dhcp server.
ETH=-netdev bridge,id=net0,br=br0 -device virtio-net-pci,netdev=net0

all:
	cd ../common/ && make && cd ../docker_version     # compile "init"
	cp -p ../common/target/x86_64-unknown-linux-musl/release/${PROGNAME} init
	docker build -t $(IMGNAME) .

run:
	 docker run -d -p 7878:7878 $(IMGNAME)

clean:
	rm -f init
	for dock in `docker ps --all | grep /sbin/init | cut -d ' ' -f 1`; do docker kill $$dock; done
	docker image remove kernel-initonly -f
	cd ../common/ && make clean
