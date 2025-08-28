Minimal example that resembles the system setup I have in my game, and recreates the issue I'm facing.

I spawn two entities when a shot is fired;
- The projectile itself which has a system to raycast and move (Raycasting not included in this example) 
- The particle effect, which has it's transform updated to match the projectile every frame.

I do this so when a projectile is destroyed upon hitting something the ribbon remains to be cleaned up by another system shortly after.

Projectile speed can be adjusted with up/down arrows to demonstrate that the ribbon stars further away from origin with a faster speed.
Also notice on occasion (Seemingly randomly, maybe 10-20% of the time), sometimes the ribbon does start from the circle as I would expect.
This would make me think it's a scheduling issue with tracking the projectile, though both the systems for moving the projectil entity & the trail are chained. 
