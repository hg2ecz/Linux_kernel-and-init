DOCKER_IMGNAME=kernel-initonly
PROGNAME=init

all:
	cd ../common/ && make            # compile "init"
	cp -p ../common/target/x86_64-unknown-linux-musl/release/${PROGNAME} init
	docker build -t $(DOCKER_IMGNAME) .

run:
	 docker run -d -p 7878:7878 $(DOCKER_IMGNAME)

clean:
	rm -f init
	docker ps -a -q --filter ancestor=$(DOCKER_IMGNAME) | xargs -r docker stop
	docker ps -a -q --filter ancestor=$(DOCKER_IMGNAME) | xargs -r docker rm
	docker image rm $(DOCKER_IMGNAME)
	cd ../common/ && make clean
