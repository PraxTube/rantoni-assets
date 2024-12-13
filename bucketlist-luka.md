# Luka's Bucketlist

## TODO

- Update to Bevy 0.15 once all deps are up to date
    - This may breake quite a lot, so this would be kinda a pain, but it would also solve some issues
- Make player non-staggerable when performing certain attacks (dropkick, hammerfist, the heavy attacks?), though still take damage
- Attack buffer (cayote buffer), the game should be buffering inputs, attack input, for example when performing a dash, and the player presses the attack button just before the dash ends (so while the player state is still dashing) then the input will be flat out ignored. This can be quite unsatisfaying and frustrating. Maybe you even want to have a separate dash attack (like in Hades)? Though that might be a bit of a pain in the ass to implement (really not that hard, but you would need to make some more animations). Regardless of that, you should definitely handle that attack input somehow and perform the attack after the dash is finished.

## Polish

- Probably want to squash enemies (insta kill) when you dropkick them and they are at a wall (even if their health isn't 0), this would also fix the issue of the weird jitter that happens when you push an enemy into a wall but then he pushes you back.
- Add damage highlight (red vignette? How does Redeemer do it?)
- Add custom (big, good visible) mouse cursor, like in wizards of legends or redeemer `https://github.com/bevyengine/bevy/pull/14284`
- Send enemies flying when landing killing blow and there is no obstacle behind them
    - Do you want to send them flying like in anime in a comical manner where they do barrel roll (just watch this `https://youtu.be/Iei_44Lz99w?t=60`)
- Also you can contiously spawn blood drops when enemies are flying away on killing blow, those drops then land on the ground and create a pile, which in turn creates a blood trail which could look pretty cool

## Maybe Features?

- Increase player hurtbox when parrying? It can feel really shit when the enemies just barely miss the player when he wants to parry an attack. You could easily just create a new HurtboxType.
- Dynamic static shadows? Predifined shadows of various sizes that can be changed based on the animation that is playing, like wizards of legends does it. So different shadow for idle and running for example.
- Aim assist (auto lock)? Redeemer does it an it feels pretty good, Hades on the other hand just has huge ass hitboxes for player attacks and that also feels great.
- Read in player parameters from a script (a simple human readable file like a .ron file). This would allow others to easily test different parameters and discover a more juicy player movement. It would also reduce magic numbers in the code. Do we want to also do this for enemies?

## Fix Ups

- NVIDIA + Wayland causes issues, see `https://github.com/bevyengine/bevy/discussions/14810`, hopefully fixed in Bevy 0.15
- Do you want to fix up the AI movement when doing diagonals? So that the bucketing works more smoothly?
