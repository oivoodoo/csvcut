# Build the Amazon Linux 2 version of the binary using Docker
build-amzn2:
    docker build -t csvcut-amzn2-builder -f Dockerfile.amzn2 .
    docker create --name temp-container csvcut-amzn2-builder
    docker cp temp-container:/usr/local/bin/csvcut ./csvcut-amzn2
    docker rm temp-container

# Clean up Docker images and containers
clean:
    docker rmi csvcut-amzn2-builder || true
    docker rm temp-container || true