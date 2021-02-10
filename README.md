# Center Brain Archive

a recipe viewer for Dyson Sphere Program

[You can find the site here.](https://gamma-delta.github.io/center-brain-archive/)

## How to Use

By default, usage links are on, so all items come with helpful links for ways to produce and consume them.

If you like, you can disable those links with the slider in the upper-right. In this mode,
left-click on an item to see all the ways to produce it and right-click to see all the ways to consume it.
(Fans of JEI will recognize these controls.)

In the future I hope to add:

- Technology tree (It knows about the whole tech tree, but just doesn't display it)
- Item info (burn energy, stack size...)
- A search bar

## How to Contribute

1) Have Rust, Nodejs, and Typescript installed.
2) `git clone` this
3) `cargo run` the `/generator` folder to generate `dsp.json` and `dsp.d.ts`

### Repository Structure

The `generator` folder contains Rust code which generates a JSON file and a `.d.ts` file.
The JSON has all the data and the `.d.ts` has definitions for it.

The `site` folder has the frontend Typescript code to display it.

The Github Page is actually published out of the `gh-pages` branch. (If you want to contribute to this site,
you likely don't need to worry about that branch because it's generated automatically.)

## Credits

- @mellester for writing a script to scrape DSP recipes
- Factoriolab for [the item sprites](https://github.com/factoriolab/factorio-lab/blob/master/src/data/dsp/icons.png)
- The DSP Wiki maintainers
- The DSP Discord
- And of course, Youthcat Studio. 您们的电子游戏真牛！
