# Co-op Cloud Docker Apps

A library to list running apps on this server (or server cluster) installed with [Co-op Cloud](https://coopcloud.tech/).

Co-op Cloud runs apps using Docker Swarm, and so in practice this means finding running "stacks" in docker swarm, and their services. Co-op Cloud has some conventions for docker swarm service labels, and these will be parsed, but apps which do not follow these conventions will return something.

## License

This library is licensed under the [The Anti-Capitalist Software License, v1.4](https://anticapitalist.software/).

If you're a community run software project that just needs a different license, please have a human drop us a line by filing a [github issue](https://github.com/local-resilience-tech/lores-node) and we can sort you out.
