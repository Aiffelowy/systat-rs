# systat-rs

a small status bar for tmux.

you can build it with:
```
cargo build --release
```
and then add 
```
set -g status-<right or left> "#(<exec path>)"
``` 

to your tmux.conf

to change colors edit main.rs and change the values. maybe I will add config file someday.
