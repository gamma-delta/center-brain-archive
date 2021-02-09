# Center Brain Archive

a recipe viewer for Dyson Sphere Program

[You can find the site here.](https://gamma-delta.github.io/center-brain-archive/)

## How to Use

Left-click on an item to see all the ways to produce it. Right-click to see all the ways to consume it.
(Fans of JEI will recognize these controls.)

Most recipes are NYI because it takes a long time to input them, so this tool is more-or-less useless right now.
Sorry... but feel free to open a PR and add them!

In the future I hope to add:

- A way to save recipes and pin them to the side somehow
- Technology tree (It knows about the whole tech tree, but just doesn't display it)

## How to Contribute

1) Have Rust, Nodejs, and Typescript installed.
2) `git clone` this
3) `cargo run` the `/generator` folder to generate `dsp.json` and `dsp.d.ts`

Check out the `help-wanted` issue label! (Most of it is just grindy slow data copying.)

### Repository Structure

The `generator` folder contains Rust code which generates a JSON file and a `.d.ts` file.
The JSON has all the data and the `.d.ts` has definitions for it.

The `site` folder has the frontend Typescript code to display it.

The Github Page is actually published out of the `gh-pages` branch. (If you want to contribute to this site,
you likely don't need to worry about that branch because it's generated automatically.)

## Credits

- @mellester for writing a script to scrape DSP recipes
- The DSP Wiki maintainers
- The DSP Discord
- And of course, Youthcat Studio. 您们的电子游戏真牛！
