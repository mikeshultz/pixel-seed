# Pixel Seed

A fun little program that procedurally generates SVGs from a given 32-byte hex
string seed.

```bash
./pixel-seed 0x3950a1e52b4feb3de8d2b67edf0bd135c58c5a04fef43d6c6a3154321aeb712f
```

**NOTE**: As of this writing, this will drop an image at `./tmp/pixel_seed_test.svg`.

## Seed

This will probably be altered a bunch going forward.

```
 Background RGBA                                       Foreground RGBA
 |        |                                            |          |
0x3950a1e52b4feb3de8d2b67edf0bd135c58c5a04fef43d6c6a3154321aeb712f
         |                                              |
         Seed for path generation ----------------------+
```
