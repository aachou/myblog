+++
title = "Understanding Linux Namespaces and Container Isolation"
date = "2023-06-22"
tags = ["linux", "containers", "kernel"]
excerpt = "Linux namespaces are the building blocks of container isolation. This deep dive explains the seven namespace types and how they isolate processes from one another."
+++

Containers feel like lightweight virtual machines, but the magic happens at the kernel level. Linux namespaces wrap global system resources in an abstraction that makes processes within a namespace see their own isolated instance.

## What Are Namespaces?

Namespaces partition kernel resources such that one set of processes sees one set of resources while another set sees a different set. There are currently seven namespace types:

| Namespace | Isolates | Created with |
|-----------|----------|-------------|
| Mount (mnt) | Filesystem mount points | CLONE_NEWNS |
| Process ID (pid) | Process IDs | CLONE_NEWPID |
| Network (net) | Network stacks | CLONE_NEWNET |
| Inter-process comm (ipc) | IPC resources | CLONE_NEWIPC |
| UTS | Hostname and domain | CLONE_NEWUTS |
| User (user) | User and group IDs | CLONE_NEWUSER |
| Cgroup (cgroup) | Cgroup root directory | CLONE_NEWCGROUP |

## Creating Namespaces with clone()

The `clone()` syscall accepts flags that create processes in new namespaces:

```c
#define _GNU_SOURCE
#include <sched.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/wait.h>

static int child_func(void *arg) {
    printf("Child PID: %d\n", getpid());
    return 0;
}

int main() {
    char stack[1024 * 1024];
    pid_t pid = clone(child_func, stack + sizeof(stack),
                      CLONE_NEWPID | CLONE_NEWUTS | SIGCHLD, NULL);
    if (pid == -1) {
        perror("clone");
        return 1;
    }
    waitpid(pid, NULL, 0);
    return 0;
}
```

## Using unshare()

The `unshare()` syscall moves the calling process into a new namespace:

```bash
# Run a shell with new UTS, PID, and mount namespaces
unshare --fork --pid --mount --uts bash

# Inside the namespace
hostname isolated-container
echo $$
# PID is 1 inside the new PID namespace
```

## How Docker Uses Namespaces

Docker leverages all seven namespace types plus cgroups for resource limits:

1. **Mount namespace** — Each container gets its own filesystem hierarchy.
2. **PID namespace** — Processes inside see PID 1; host sees the real PID.
3. **Network namespace** — Each container has its own network interfaces and routing tables.
4. **User namespace** — Root inside maps to a non-privileged user outside.

```text
Host View:       Container View:
PID 2345 ---\    PID 1
PID 2346 ---+--> PID 2
PID 2347 ---/    PID 3
```

## User Namespaces and Security

User namespaces are the most important for security. A process can run as root inside a user namespace while having no privileges on the host:

```bash
unshare --user --map-root-user bash
whoami  # root
id -u   # 0
# But on the host, this process runs as an unprivileged user
```

## Limitations

Namespaces do not provide complete isolation. The host kernel is still shared, so kernel vulnerabilities can escape container boundaries. This is why defense-in-depth is critical — use seccomp, AppArmor, and read-only root filesystems alongside namespaces.

Understanding namespaces demystifies containers. They are not virtualization — they are clever kernel primitives that give the illusion of isolation without the overhead of a hypervisor.
