from discord import Intents, Client, Interaction, Game
from discord.app_commands import CommandTree
import discord
import requests
import subprocess
import os
from os.path import join, dirname
from dotenv import load_dotenv

load_dotenv(verbose=True)

dotenv_path = join(dirname(__file__), '.env')
load_dotenv(dotenv_path)

# Discord botのトークンを、左欄の「TOKEN」と置き換えてください
TOKEN = os.environ.get("DICORD_TOKEN")

SERVER_ADDRES = os.environ.get("SERVER_ADDRES")
SERVER_ADDRES = "mc.mochi33.com"
SERVER_PATH = "minecraft/java/paper"  # Minecraftサーバー実行ファイルのあるフォルダのパスを入力してください
SHELL_FILE = "start.sh"  # Minecraftサーバーを起動するシェルファイルの名前を入力してください
SCREEN_NAME = "steame_punk_mc"  # 使用するscreenセッションの名前を入力してください


# 以下はコードです。必要に応じていじってください


class MyClient(Client):
    def __init__(self, intents: Intents) -> None:
        super().__init__(intents=intents)
        self.tree = CommandTree(self)

    async def setup_hook(self) -> None:
        await self.tree.sync()

    async def on_ready(self):
        # 「○○をプレイ中」と表示するところ
        await client.change_presence(activity=Game(name="/startでサーバーを起動"))
        print(f"login: {self.user.name} [{self.user.id}]")


intents = Intents.default()
client = MyClient(intents=intents)


# コマンドの名前と、コマンドの説明です
@client.tree.command(name="hello", description="Hello, world!")
async def hello(interaction: Interaction):
    await interaction.response.send_message(f'Hello, {interaction.user.mention}!')


# コマンドの名前と、コマンドの説明です
@client.tree.command(name="start", description="サーバーを起動する")
async def start(interaction: Interaction):
    if is_server_running():
        await interaction.response.send_message('サーバーは既に起動しています')
    else:

        start_server()
        await interaction.response.send_message('サーバーを起動します')


# コマンドの名前と、コマンドの説明です
@client.tree.command(name="get_status", description="サーバーの状態を確認する")
async def get_status(interaction: Interaction):

    api_url = "https://api.mcstatus.io/v2/status/java/" + SERVER_ADDRES
    response = requests.get(api_url)
    json_response = response.json()
    print(json_response["online"])

    if json_response["online"]:
        # await interaction.response.send_message('mc.mochi33.com : サーバーは起動しています')
        embed = discord.Embed(
            title="マイクラサーバーのステータス",
            description="",
            color=0x2ECC71,
           )
        embed.add_field(name="ステータス", value="起動中 🟢" , inline=False)
        embed.add_field(name="プレイヤー数", value=json_response["players"]["online"],  inline=False)
        name_list = ""
        for i in json_response["players"]["list"]:
            name_list += i["name_raw"] + "\n"
        embed.add_field(name="プレイヤー一覧", value=name_list,  inline=False)


        await interaction.response.send_message(embed=embed)

    else:
        embed = discord.Embed(
            title="マイクラサーバーのステータス",
            description="",
            color=0xE74C3C
           )
        embed.add_field(name="ステータス", value="停止中")  # フィールドを追加。


        await interaction.response.send_message(embed=embed)


def is_server_running():  # サーバーが動作しているか確認する関数です
    process=subprocess.Popen(
        f"screen -ls {SCREEN_NAME}", stdout=subprocess.PIPE, shell=True)
    output, _=process.communicate()
    return SCREEN_NAME in output.decode()


def start_server():  # screenを利用してサーバーを起動するコマンドは以下になります
    subprocess.Popen(f"screen -dmS {SCREEN_NAME} ./start.sh", shell=True)


client.run(TOKEN)
client.run(TOKEN)
