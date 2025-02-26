### When the connect losts, it shuts down.

When running, it will loop and loop, ping the ip or website you set every minutes(by default).  
When the connect losts, it **won't** shutdown immediately, but turn in **emergency-loop** mode. This time, it will ping every 20 seconds(also default), if still nothing for 1 minutes, it shuts down.

Available for Windows and Unix-like systems(and MacOS)

Use **cargo build --release** to build.

### You can also set some advanced settings.
~~~
Usage: main [OPTIONS]

Options:
  -i, --ip <IP>
          the ip address or website you want to check [default: bing.com]
  -a, --and-or <AND_OR>
          use -o to active shutdown when any connection losts [default: None]
  -n, --secs-for-normal-loop <SECS_FOR_NORMAL_LOOP>
          time between two normal check [default: 60]
  -e, --secs-for-emergency-loop <SECS_FOR_EMERGENCY_LOOP>
          time between two emegency check [default: 20]
  -t, --times-for-emergency-loop <TIMES_FOR_EMERGENCY_LOOP>
          times for emergency lopp [default: 3]
  -h, --help
          Print help
  -V, --version
          Print version
~~~

**Please please ignore my poor English(⋟﹏⋞)**

Have a nice day. ヾ(✿ﾟ▽ﾟ)ノ