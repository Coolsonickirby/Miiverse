from bs4 import BeautifulSoup
import wget
with open("tmp.html", encoding="utf-8") as fp:
    soup = BeautifulSoup(fp, "html.parser")
imgs = soup.find_all("img", class_="post-memo")

imgs = imgs[0:30]
for x in range(len(imgs)):
    wget.download(imgs[x].get('src'), out=f"imgs/{x}.png")