# Remote Canvas

An adapter for Browser-based canvas implementations into a Rust application at scale.

## Goals

* Provide a scalable way to use a browser-based Canvas implementation in Rust applications.
* Take advantage of multiple browsers at the same time
* Provide a nearly-identical API to 2D context rendering.

## Future Goals

* Provide a nearly-identical API to WebGL context rendering.
* Integrate with local Chrome install or a Docker cluster of Chrome containers in order to provide a connected browser on demand.

## Limitations

* A browser may disappear at any time which will destroy all attached Canvas' at the API level. It is up to the end user to restart their context.
    * This is intentional as, while tracking the context transactionally to restart it is relatively easy, it may have significant performance issues.
* Unless load is sufficiently high, only one browser will be used.
    * This is due to the use of `async_std`'s MPMC `channel` versus a proper load balancing solution.
    * A long term solution may be a load balance job.
* No browser authentication