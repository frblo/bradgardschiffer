# bradgardschiffer
Small program for generating ascii brädgårdschiffer strings.

![Brädgårdschiffer](https://web.cdn.scouterna.net/uploads/sites/57/2019/11/500px-chiffer_bradgard.png)

As can be seen the letters `w` and `q` are not able to be printed.

## Examples

### Small string

Key: `Hello`

Output: 
```
+---+ +---+ |     |     +----
|   | |   | | *   | *   | *
|   | +---+ +---- +---- +----
```

### Larger string

Key: `brädgårdschiffer is practical`

Output:
```
|   | +---+ +---+ ----+ ----+ ----+ +---+ ----+ +---- |     +---+ +---- +---- +---- +---+ +---+ +---- +---- ----+ +---+     | |       * | +---- |         | |
|   | | * | | * |     |     |   * | | * |     | | *   |     |   | |     |     |     |   | | * | |     | *       | | * |     | |       * | |     |         | | *
+---+ |   | | * | ----+     | *   | |   | ----+ |     +---- |   | |     +---- +---- +---+ |   | |     |       * | |   | ----+ +---- ----+ |     +---- ----+ +----
```