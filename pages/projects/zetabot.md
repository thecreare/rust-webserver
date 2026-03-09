# Zetabot

Discord bot made with Discord.py

![Cursive letter Z on background of minified javascript code](https://cdn.discordapp.com/avatars/798634937606864937/fba06420b1ed2c736e591b5dc5ec5277?size=128)

I am not proud of this project by any means but it was a lot of firsts for 14 year old me. It used a PostgreSQL database (rip ElephantSQL), which of course had *numerous* 😭 sql injection vulnerabilities, luckily nobody found those before I lost the ability to host it (rip Heroku)

If anyone had noticed, it would've been as simple as running `>rankimg ;DROP SCHEMA public CASCADE;--` to drop every table. The *truly* amazing code that made that possible:

```py
@commands.command(name='rankimg', help='Changes your rank card background')
async def rank_url(self, ctx, quote: str = None):
    if(quote):               
        id = ctx.author.id
        await db.query(
            f'''
                UPDATE users
                SET imgurl = \'{quote}\'
                WHERE id = {id};
                ''')
            await ctx.send("Done, set image!")
        else:
            await ctx.send("Please specify a img url, eg `rankimg <url>`")
```

Beyond the sql injection, there were loads of other problems. The bot created a new database table for every player's inventory and every discord server it joined. Maintaining that was a mess.

Since then I've learned a lot and i'm happy to say I don't make these mistakes anymore lol.

As a fun fact, ElephantSQL offered a "free plan" with a tiny amount of storage, my bot surpassed the limit many times over and never stopped functioning. I don't think the limit was real...

The bot had some neat features, I took a lot of inspiration from other popular bots. There was an XP & leveling system (ack, I hate those now 😭), an inventory, crafting, resource gathering, moderation, and other random silly commands

There was a neat feature called "Global Economy" which was probably code for "I didn't want to store player data per-server" that meant player's inventories where shared through all servers.
