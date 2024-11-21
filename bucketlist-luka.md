# Luka's Bucketlist

## TODO

- Update bevy_trickfilm once the warn -> debug
- Make player non-staggerable when performing certain attacks (dropkick, hammerfist, the heavy attacks?)
- Probably want to squash enemies when you dropkick them and they are at a wall (even if their health isn't 0), this would also fix the issue of the weird jitter that happens when you push an enemy into a wall but then he pushes you back.
- Attack buffer (cayote buffer), the game should be buffering inputs, attack input, for example when performing a dash, and the player presses the attack button just before the dash ends (so while the player state is still dashing) then the input will be flat out ignored. This can be quite unsatisfaying and frustrating. Maybe you even want to have a separate dash attack (like in Hades)? Though that might be a bit of a pain in the ass to implement (really not that hard, but you would need to make some more animations). Regardless of that, you should definitely handle that attack input somehow and perform the attack after the dash is finished.

## Maybe Features?

- Increase player hurtbox when parrying? It can feel really shit when the enemies just barely miss the player when he wants to parry an attack. You could easily just create a new HurtboxType.
- Dynamic static shadows? Predifined shadows of various sizes that can be changed based on the animation that is playing, like wizards of legends does it. So different shadow for idle and running for example.
- Aim assist (auto lock)? Redeemer does it an it feels pretty good, Hades on the other hand just has huge ass hitboxes for player attacks and that also feels great.
- Read in player parameters from a script (a simple human readable file like a .ron file). This would allow others to easily test different parameters and discover a more juicy player movement. It would also reduce magic numbers in the code. Do we want to also do this for enemies?
