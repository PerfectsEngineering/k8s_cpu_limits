What?
- [x] How to know how many CPU is available in Rust? 
ANS: used the num_cpus crate (because it was easier)
- [x] How does this show up in a Kubernetes workload container?
ANS: It is using the `resources.limit`
- [x] Can I run multiple threads in less than on 1vCPU successfully?
ANS: Yes, because of CPU throttling.
