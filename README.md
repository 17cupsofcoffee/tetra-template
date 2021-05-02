# tetra-template

This repo contains an example of a blank [Tetra](https://github.com/17cupsofcoffee/tetra) project.

It comes with some pre-built functionality and opinionated defaults, based on what I usually
want/need when I'm starting a new project:

* Simple asset loading
* A hotkey to reload assets
* Screen scaling, for chunky pixel graphics
* Pre-multiplied alpha
* Error reporting via [eyre](https://github.com/yaahc/eyre)

Feel free to use this as a starting point for your own games!

## FAQ

### Why are your assets in a seperate struct?

Assets like textures and sounds are reference-counted internally, so it can be tempting to scatter copies of them across your game. However, I've found it's usually better to try to keep them in one place, so you can easily update/reload them without needing to propagate that change throughout your game's state. I'll then pass this `Assets` struct around where it's needed (e.g. for drawing/playing sounds).

There are probably more robust solutions for asset management, but I think this is a good starting point for simple projects!

### Why do you pre-multiply textures at runtime?

This makes it so I can edit my textures and reload them while the game is running, without a recompile or extra build step. This massively reduces the friction of working with premultiplied alpha for me.

For textures that *are* already pre-multiplied, I use a `.pm` naming convention, which tells the asset loader to skip the runtime conversion.