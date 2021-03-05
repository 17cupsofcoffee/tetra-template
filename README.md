# tetra-template

This repo contains an example of a blank [Tetra](https://github.com/17cupsofcoffee/tetra) project.

It comes with some pre-built functionality, based on what I usually want/need when I'm starting a new project:

* Simple asset reloading via a hotkey
* Screen scaling, for chunky pixel graphics

Feel free to use this as a starting point for your own games!

## FAQ

### Why are your assets in a seperate struct?

Assets like textures and sounds are reference-counted internally, so it can be tempting to scatter copies of them across your game. However, I've found it's usually better to try to keep them in one place, so you can easily update/reload them without needing to propagate that change throughout your game's state. I'll then pass this `Assets` struct around where it's needed (e.g. for drawing/playing sounds).

There are probably more robust solutions for asset management, but I think this is a good starting point for simple projects!
