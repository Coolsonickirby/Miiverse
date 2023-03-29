from http.server import BaseHTTPRequestHandler, HTTPServer
import cv2, os, json, random, time

def divide_chunks(l, n):
    # looping till length l
    for i in range(0, len(l), n): 
        yield l[i:i + n]
        
hostName = "10.0.0.42"
serverPort = 6920
colors = [
    (54, 54, 254),   # Player 1 Color
    (255, 137, 46),  # Player 2 Color
    (16, 187, 255),  # Player 3 Color
    (72, 180, 40),   # Player 4 Color
    (54, 134, 249),  # Player 5 Color
    (234, 210, 44),  # Player 6 Color
    (180, 155, 255), # Player 7 Color
    (255, 112, 149), # Player 8 Color
    (172, 172, 172), # CPU Player Color
]

character_map = {"0": "mario","1": "donkey","2": "link","3": "samus","4": "samusd","5": "yoshi","6": "kirby","7": "fox","8": "pikachu","9": "luigi","10": "ness","11": "captain","12": "purin","13": "peach","14": "daisy","15": "koopa","16": "sheik","17": "zelda","18": "mariod","19": "pichu","20": "falco","21": "marth","22": "lucina","23": "younglink","24": "ganon","25": "mewtwo","26": "roy","27": "chrom","28": "gamewatch","29": "metaknight","30": "pit","31": "pitb","32": "szerosuit","33": "wario","34": "snake","35": "ike","36": "pzenigame","37": "pfushigisou","38": "plizardon","39": "diddy","40": "lucas","41": "sonic","42": "dedede","43": "pikmin","44": "lucario","45": "robot","46": "toonlink","47": "wolf","48": "murabito","49": "rockman","50": "wiifit","51": "rosetta","52": "littlemac","53": "gekkouga","54": "palutena","55": "pacman","56": "reflet","57": "shulk","58": "koopajr","59": "duckhunt","60": "ryu","61": "ken","62": "cloud","63": "kamui","64": "bayonetta","65": "inkling","66": "ridley","67": "simon","68": "richter","69": "krool","70": "shizue","71": "gaogaen","72": "miifighter","73": "miiswordsman","74": "miigunner","75": "iceclimber","76": "iceclimber","77": "koopag","78": "miienemyf","79": "miienemys","80": "miienemyg","81": "packun","82": "jack","83": "brave","84": "buddy","85": "dolly","86": "master","87": "tantan","88": "pickel","89": "edge","90": "eflame","91": "elight","92": "demon","110": "ice_climber","111": "zenigame","112": "fushigisou","113": "lizardon","114": "ptrainer"}

def addBorder(image, color):
    row, col = image.shape[:2]
    bordersize = 5
    border = cv2.copyMakeBorder(
        image,
        top=bordersize,
        bottom=bordersize,
        left=bordersize,
        right=bordersize,
        borderType=cv2.BORDER_CONSTANT,
        value=color
    )
    return border

def generatePosts(data):
    print(data)
    read_files = []
    imgs = []
    posts_per_player = int(64 / len(data["players"]))
    for x in data["players"]:
        path = f"./imgs/{character_map.get(str(x['player_chara']), x['player_chara'])}"
        for y in range(posts_per_player):
            try:
                i = 0
                file_path = f"{path}/{random.choice(os.listdir(path))}"
                while file_path in read_files and i < 50:
                    file_path = f"{path}/{random.choice(os.listdir(path))}"
                    i += 1
                read_files.append(file_path)
                imgs.append(addBorder(cv2.imread(file_path), colors[-1] if x["is_cpu"] or x["is_amiibo"] else colors[x["player_num"]]))
            except:
                file_path = "./imgs/none.png"
    random.shuffle(imgs)
    final_img = []
    test = list(divide_chunks(imgs, 8))
    for x in test:
        hconcat = []
        for y in x:
            hconcat.append(y)
        final_img.append(cv2.hconcat(hconcat))
    return cv2.vconcat(final_img)

class MyServer(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "text/html")
        self.end_headers()
        self.wfile.write(bytes("tmp", encoding="utf-8"))
    def do_POST(self):
        content_len = int(self.headers.get('Content-Length'))
        post_body = self.rfile.read(content_len)
        image = cv2.imencode('.png', generatePosts(json.loads(post_body)))[1].tobytes()
        self.send_response(200)
        self.send_header("Content-type", "image/png")
        self.send_header("Content-length", len(image))
        self.end_headers()
        self.wfile.write(image)

if __name__ == "__main__":        
    webServer = HTTPServer((hostName, serverPort), MyServer)
    print(f"Server started http://{hostName}:{serverPort}")
    webServer.serve_forever()
    webServer.server_close()
    print("Server stopped.")