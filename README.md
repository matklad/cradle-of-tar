# Cradle Of Tar

This is a short demonstration of an argument injection vulnerability.
Inspired by https://staaldraad.github.io/post/2019-11-24-argument-injection/.

`cradle-of-tar` is a convenience wrapper over tar, which just compresses a single directory:

```console
$ ls
drwxr-xr-x   - matklad 16 Sep 08:31 src
$ cradle-of-tar ./src
$ ls
drwxr-xr-x   - matklad 16 Sep 08:31 src
.rw-r--r-- 10k matklad 16 Sep 08:52 archive.tar
drwxr-xr-x   - matklad 16 Sep 08:31 src
```
The core of the implementation is this line:

```rust
run!(%format!("tar -cf archive.tar {}", directory));
```

The problem here is that, if `directory` contains whitespace, this will pass more than one extra argument to `tar`.
And `tar` has arguments which allow executing arbitrary commands.
For example, the following invocation will reboot your computer:

```
$ cradle-of-tar 'src -Ireboot'
```
