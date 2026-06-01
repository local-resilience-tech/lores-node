setup:
    cd frontend && npm install

dev:
    mprocs

docker:
    #!/usr/bin/env bash
    set -e
    docker build -t lores-node .
    trap 'docker stop lores-node' INT TERM EXIT
    docker run --rm --name lores-node -p 3000:3000 lores-node &
    echo "Press Control-C 3 times to exit"
    wait

two-node:
    mprocs --config mprocs-2node.yaml