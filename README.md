## When the connect losts, it shuts down.

When running, it will loop and loop, ping the ip or website you set every minutes(by default).  

When the connect losts, it **won't** shutdown immediately, but turn in **emergency-loop** mode. This time, it will ping every 20 seconds(also default), if still nothing for 1 minutes, it shuts down.

Available for Windows and Unix-like systems(and MacOS)[Now trying to supporrt ESXI]

Use **cargo build --release** to build.

### You can also set some advanced settings.
~~~
Usage: pingdown [OPTIONS] [IP, WEBSITE]...

Arguments:
  [IP, WEBSITE]...  the ip address or website you want to check

Options:
  -s, --strict                       Active strict mode. It will shutdown when any connection losts
  -n <SECS_FOR_NORMAL_LOOP>          time between two normal check [default: 60]
  -e <SECS_FOR_EMERGENCY_LOOP>       time between two emegency check [default: 20]
  -t <TIMES_FOR_EMERGENCY_LOOP>      times for emergency lopp [default: 3]
  -h, --help                         Print help
  -V, --version                      Print version
~~~

**Please please ignore my poor English(⋟﹏⋞)**

Have a nice day. ヾ(✿ﾟ▽ﾟ)ノ