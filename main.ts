import { createBot, Intents, startBot } from "discorddeno";
import "https://deno.land/std@0.205.0/dotenv/load.ts";
import {
  enableCachePlugin,
  enableCacheSweepers,
} from "https://deno.land/x/discordeno_cache_plugin@0.0.21/mod.ts";

const baseBot = createBot({
  token: Deno.env.get("DISCORD_TOKEN")!,
  intents: Intents.Guilds | Intents.GuildMessages | Intents.MessageContent,

  events: {
    ready() {
      console.log("Successfully connected to gateway");
    },
    messageCreate(bot, message) {
      if (message.content.startsWith("!ping")) {
        bot.helpers.sendMessage(message.channelId, { content: "pong!" });
      }
    },
  },
});

await startBot(baseBot);
