# SNS Log Puller

This is my personal project I made to make it easier to pull logs onto a machine. This can be run from the terminal,
which leads to its rapid prototyping.

This is in very early stages in an attempt to get something out the door quickly with some functionality. As such,
there's lots of ugly code. However, here is what I plan to do:

## TODO
1. Parameterize the main function. Allow specifying a log file and a canister.
2. Better error handling across the board. Try to recover errors.
3. Separate the library from the means of running it. Have a thin runtime environment that is setup by a CLAP bin.
4. Have more efficient flushing of files. 
5. Unit testing.
6. Have more efficient queueing mechanisms.