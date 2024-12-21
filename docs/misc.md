# Misc

## Legal

### Localization

So this happened to Jonas `https://youtu.be/MaFpf3nmHmo?t=798`, I personally think that if you had a github repo with a clear license like MIT that said clearly that anybody that would contribute will have their work licensed under MIT would make this essentially a non problem? Though I guess it would be better to ask a lawyer for that? But if this were a problem then wouldn't all of open source be at risk?

## Feedback

- So mock reviews are a thing apparently, see `https://youtu.be/g-geVoA8Xr4?t=793`
- Look at negative reviews on similar games, see `https://youtu.be/g-geVoA8Xr4?t=968`

## Random Shit

- `Anthony the Angry`, a boss that doesn't really have much to do with the story or anything else. Model based on Anton. Before the "boss fight" there is a bit of dialogue, bla bla, and then the fight begins, boss bar appears, epic music starts, Anthoni charges at you full speed, maybe even yelling in a local dialogue box a battle cry... then he trips and falls, puddle of blood starting to spread, his health bar goes from full to zero instantly, music cuts off. Thus, `Anthoni the Angry` was defeated, by the floor. So it's a fake boss battle.
- Oh yeah what about a fake puzzle? Maybe that's too much work.

## Optimization

The biggest concern is memory. I would really like to stay below 2 gigs, and the biggest offender of this are the characters (player, enemies etc). There are two optimizations I can implement fairly easily that should reduce memory by more than 50%.

- Batch frames that are identical together and utilize trickfilms keyframe timestamps to encode the same frame data. Useful for stuff like idle animations or parries or later on enemy anticipation animations. **Expected Result: ~15**
- Really just don't load everything in at once. If you have enemies specific to one area only then only load them in at that area.

## Playtesting, QA

Testing niddy griddy like player stats (movement speed, attack speed bla bla bla) can be easily done by having a txt file or ron file and letting the testers just adjust those values and then hot reload the game state to reflect the changes.

Also allows for easily copy/pasting configs.

Future me here, is that really necessary?
