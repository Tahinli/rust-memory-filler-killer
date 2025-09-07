# rust-memory-filler-killer
Finds and Kills Memory Filler

## Why I did this
Sometimes Linux kernel is not able to kill memory consumers when memory reaches the critical level. I don't know why but I personally lived this situations couple of times and my computer couldn't stay stable so I had to restart it. That's why I implemented this.

> ./rust-memory-filler-killer -h

<img src=assets/help.png>

> ./rust-memory-filler-killer --control_delay 1000 --dealloc_delay 1000 --kill_threshold 0.95 --include_swap false

<img src=assets/example.png>
