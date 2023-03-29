from bs4 import BeautifulSoup
import wget, os

def makeDir(path):
    try:
        os.mkdir(path)
    except:
        pass

for x in os.listdir("."):
    if x[-4:] == "html" and os.path.isfile(x):
        with open(x, encoding="utf-8") as fp:
            soup = BeautifulSoup(fp, "html.parser")
        imgs = soup.find_all("img", class_="post-memo")

        charaName = os.path.basename(x).split('.')[0]
        print(charaName)
        print(len(imgs))
        print("----------------")
        imgs = imgs[0:50]
        makeDir(f"out/{charaName}")
        for x in range(len(imgs)):
            src = imgs[x].get('src')
            try:
                wget.download(src, out=f"out/{charaName}/{x}.png")
            except:
                print()
                print(f"Failed downloading {src} for Index {x} Chara {charaName}")
        print()