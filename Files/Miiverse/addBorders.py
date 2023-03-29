from random import choice
import os
colors = ["red", "blue", "green", "orange", "purple", "cyan"]

borderedPath = "imgBordered"
srcDir = "imgs"
borderThickness = 5

try:
    os.mkdir(borderedPath)
except:
    pass

for x in os.listdir(srcDir):
    os.system(f"magick convert -bordercolor {choice(colors)} -border {borderThickness} \"{srcDir}/{x}\" \"{borderedPath}/{x}\"")
