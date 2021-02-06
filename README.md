# Center Brain Archive

a recipe viewer for Dyson Sphere Program

## How to Use

Left-click on an item to see all the ways to produce it. Right-click to see all the ways to consume it.
(Fans of JEI will recognize these controls.)

(Right-click is NYI)

## How to Contribute

1) Have Rust, Nodejs, and Typescript installed.
2) `git clone` this
3) `cargo run` the `/generator` folder and run the unit test, to generate `dsp.json` and `dsp.d.ts`

### Repository Structure

The `generator` folder contains Rust code which generates a JSON file and a `.d.ts` file.
The JSON has all the data and the `.d.ts` has definitions for it.

The `site` folder has the frontend Typescript code to display it.

The Github Page is actually published out of the `gh-pages` branch. (If you want to contribute to this site,
you likely don't need to worry about that branch because it's generated automatically.)

## Credits

* The DSP Wiki maintainers
* The DSP Discord
* And of course, Youthcat Studio. 您们的电子游戏真牛！
