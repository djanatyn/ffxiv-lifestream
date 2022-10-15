# ffxiv-lifestream

```
❯ cargo build && sudo ./target/debug/ffxiv-lifestream | hexdump -C
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
found magic: 0x41a05252
00000000  52 52 a0 41 ff 5d 46 e2  7f 2a 64 4d 7b 99 c4 75  |RR.A.]F..*dM{..u|
00000010  ff 28 d9 dd 83 01 00 00  58 00 00 00 00 00 01 00  |.(......X.......|
00000020  01 00 00 00 00 00 00 00  30 00 00 00 e2 93 6c 10  |........0.....l.|
00000030  0d 67 86 10 03 00 00 00  14 00 9f 03 00 00 19 00  |.g..............|
00000040  a4 39 4b 63 00 00 00 00  82 cb 40 00 3c 00 36 48  |.9Kc......@.<.6H|
00000050  01 9b 7b 62 00 02 05 00                           |..{b....|
00000058
```

# resources
* [`Machina.FFXIV/FFXIVBundleDecoder.cs`](https://github.com/ravahn/machina/blob/3e0956ed1ee6f7b428145b121a0287b7f9cbadf4/Machina.FFXIV/FFXIVBundleDecoder.cs#L75) by [ravahn](https://github.com/ravahn)
